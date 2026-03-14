use std::path::{Path, PathBuf};
use std::sync::Mutex;
use chrono::Utc;
use uuid::Uuid;
use crate::library::db::LibraryDb;
use crate::library::fs::LibraryFs;
use crate::library::types::*;

#[derive(Debug, thiserror::Error)]
pub enum LibraryError {
    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Meeting not found: {0}")]
    NotFound(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl serde::Serialize for LibraryError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, LibraryError>;

pub struct Library {
    db: Mutex<LibraryDb>,
    fs: LibraryFs,
    prepared: Mutex<std::collections::HashMap<String, PathBuf>>,
}

impl Library {
    pub fn new(db_path: &Path, recordings_dir: PathBuf) -> Result<Self> {
        let db = LibraryDb::open(db_path)?;
        let fs = LibraryFs::new(recordings_dir)?;
        Ok(Library {
            db: Mutex::new(db),
            fs,
            prepared: Mutex::new(std::collections::HashMap::new()),
        })
    }

    #[cfg(test)]
    pub fn new_in_memory(recordings_dir: PathBuf) -> Result<Self> {
        let db = LibraryDb::open_in_memory()?;
        let fs = LibraryFs::new(recordings_dir)?;
        Ok(Library {
            db: Mutex::new(db),
            fs,
            prepared: Mutex::new(std::collections::HashMap::new()),
        })
    }

    pub fn prepare_meeting(&self) -> Result<PreparedMeeting> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let dir_name = format!("{}_{}", now.format("%Y-%m-%d"), &id[..8]);
        let dir_path = self.fs.create_meeting_dir(&dir_name)?;

        // BUG FIX: Use map_err instead of unwrap() to avoid panicking if the mutex is
        // poisoned. All other mutex locks in this file already use map_err.
        self.prepared.lock()
            .map_err(|e| {
                tracing::error!("prepared mutex lock poisoned in prepare_meeting: {}", e);
                LibraryError::Internal("prepared lock poisoned".to_string())
            })?
            .insert(id.clone(), dir_path.clone());

        tracing::info!("prepare_meeting: id={}, dir={:?}", id, dir_path);
        Ok(PreparedMeeting { id, dir_path })
    }

    /// Library tracks prepared meetings internally to avoid redundant dir_path param
    pub fn finalize_meeting(&self, id: &str, info: RecordingInfo) -> Result<Meeting> {
        let now = Utc::now();
        let title = format!("Reunião {}", now.format("%Y-%m-%d %H:%M"));

        // BUG FIX: Use map_err instead of unwrap() to avoid panicking if the mutex is
        // poisoned. All other mutex locks in this file already use map_err.
        let dir_path = self.prepared.lock()
            .map_err(|e| {
                tracing::error!("prepared mutex lock poisoned in finalize_meeting: {}", e);
                LibraryError::Internal("prepared lock poisoned".to_string())
            })?
            .remove(id)
            .ok_or(LibraryError::NotFound(format!("No prepared meeting with id {}", id)))?;

        let tracks = info.tracks.clone();
        let meeting = Meeting {
            id: id.to_string(),
            title,
            created_at: now,
            duration_secs: info.duration_secs,
            has_video: info.has_video,
            file_size: info.file_size,
            dir_path,
            tracks: tracks.clone(),
            media_status: MediaStatus::Present,
            transcription_status: TranscriptionStatus::Pending,
            chat_status: ChatStatus::NotIndexed,
        };

        // Generate thumbnail for video meetings (non-fatal on failure)
        if info.has_video {
            let video_path = meeting.dir_path.join("recording.mkv");
            let thumb_path = meeting.dir_path.join("thumbnail.jpg");
            if let Err(e) = super::thumbnail::generate_thumbnail(&video_path, &thumb_path) {
                tracing::error!("thumbnail generation failed: {}", e);
            }
        }

        // Extract full audio as OGG/Opus for browser-compatible playback (non-fatal)
        // This is the complete audio, not a preview — browsers can't play MKV natively
        {
            let recording_path = meeting.dir_path.join("recording.mkv");
            let audio_path = meeting.dir_path.join("audio.ogg");
            match std::process::Command::new("ffmpeg")
                .args(["-y", "-loglevel", "error", "-i"])
                .arg(&recording_path)
                .args(["-vn", "-codec:a", "libopus", "-b:a", "128k"])
                .arg(&audio_path)
                .output()
            {
                Ok(output) if output.status.success() => {
                    tracing::info!("audio playback file generated: {:?}", audio_path);
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    tracing::error!("audio playback generation failed: {}", stderr);
                }
                Err(e) => {
                    tracing::error!("failed to run ffmpeg for audio playback: {}", e);
                }
            }
        }

        let db = self.db.lock().map_err(|_| LibraryError::Internal("lock poisoned".to_string()))?;
        db.insert_meeting(&meeting, &tracks)?;

        Ok(meeting)
    }

    pub fn cancel_prepared(&self, id: &str) -> Result<()> {
        let dir_path = self.prepared.lock()
            .map_err(|e| {
                tracing::error!("prepared mutex lock poisoned in cancel_prepared: {}", e);
                LibraryError::Internal("prepared lock poisoned".to_string())
            })?
            .remove(id)
            .ok_or(LibraryError::NotFound(format!("No prepared meeting with id {}", id)))?;

        self.fs.delete_meeting_dir(&dir_path)?;
        tracing::info!("cancel_prepared: removed id={}, dir={:?}", id, dir_path);
        Ok(())
    }

    pub fn get_meeting(&self, id: &str) -> Result<Meeting> {
        let db = self.db.lock().map_err(|_| LibraryError::Internal("lock poisoned".to_string()))?;
        db.get_meeting(id).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => LibraryError::NotFound(id.to_string()),
            other => LibraryError::Db(other),
        })
    }

    pub fn get_meeting_detail(&self, id: &str) -> Result<MeetingDetail> {
        let meeting = self.get_meeting(id)?;

        // BUG FIX: Read transcript.json (structured JSON) instead of transcript.txt (plain text).
        // The frontend's TranscriptView component tries to JSON.parse() this field to extract
        // segments with timestamps for the clickable segment view. Reading transcript.txt
        // (plain text) always caused JSON.parse to fail, so users never saw the structured
        // transcript view with clickable timestamps.
        let transcript = if self.fs.has_artifact(&meeting.dir_path, &ArtifactKind::TranscriptJson) {
            let data = self.fs.read_artifact(&meeting.dir_path, &ArtifactKind::TranscriptJson)?;
            let json_str = String::from_utf8_lossy(&data).into_owned();
            tracing::info!("get_meeting_detail: loaded transcript.json ({} bytes) for meeting {}", json_str.len(), id);
            Some(json_str)
        } else if self.fs.has_artifact(&meeting.dir_path, &ArtifactKind::TranscriptTxt) {
            // Fallback to transcript.txt if transcript.json is not available
            let data = self.fs.read_artifact(&meeting.dir_path, &ArtifactKind::TranscriptTxt)?;
            tracing::info!("get_meeting_detail: falling back to transcript.txt ({} bytes) for meeting {}", data.len(), id);
            Some(String::from_utf8_lossy(&data).into_owned())
        } else {
            None
        };

        Ok(MeetingDetail {
            id: meeting.id,
            title: meeting.title,
            created_at: meeting.created_at,
            duration_secs: meeting.duration_secs,
            has_video: meeting.has_video,
            file_size: meeting.file_size,
            dir_path: meeting.dir_path,
            media_status: meeting.media_status,
            transcription_status: meeting.transcription_status,
            chat_status: meeting.chat_status,
            tracks: meeting.tracks,
            transcript,
        })
    }

    pub fn list_meetings(&self) -> Result<Vec<MeetingSummary>> {
        let db = self.db.lock().map_err(|_| LibraryError::Internal("lock poisoned".to_string()))?;
        Ok(db.list_meetings()?)
    }

    pub fn update_title(&self, id: &str, title: &str) -> Result<()> {
        let db = self.db.lock().map_err(|_| LibraryError::Internal("lock poisoned".to_string()))?;
        Ok(db.update_title(id, title)?)
    }

    pub fn delete_meeting(&self, id: &str, mode: DeleteMode) -> Result<()> {
        // Single lock acquisition to avoid TOCTOU race
        let db = self.db.lock().map_err(|_| LibraryError::Internal("lock poisoned".to_string()))?;
        let meeting = db.get_meeting(id).map_err(|_| LibraryError::NotFound(id.to_string()))?;

        match mode {
            DeleteMode::Everything => {
                db.delete_meeting(id)?;
                self.fs.delete_meeting_dir(&meeting.dir_path)?;
            }
            DeleteMode::MediaOnly => {
                db.update_media_status(id, MediaStatus::Deleted)?;
                self.fs.delete_media_files(&meeting.dir_path)?;
            }
        }
        Ok(())
    }

    pub fn update_transcription_status(&self, id: &str, status: TranscriptionStatus) -> Result<()> {
        let db = self.db.lock().map_err(|_| LibraryError::Internal("lock poisoned".to_string()))?;
        Ok(db.update_transcription_status(id, status)?)
    }

    pub fn update_chat_status(&self, id: &str, status: ChatStatus) -> Result<()> {
        let db = self.db.lock().map_err(|_| LibraryError::Internal("lock poisoned".to_string()))?;
        Ok(db.update_chat_status(id, status)?)
    }

    pub fn save_artifact(&self, id: &str, kind: &ArtifactKind, data: &[u8]) -> Result<PathBuf> {
        let meeting = self.get_meeting(id)?;
        Ok(self.fs.save_artifact(&meeting.dir_path, kind, data)?)
    }

    pub fn get_artifact_path(&self, id: &str, kind: &ArtifactKind) -> Result<PathBuf> {
        let meeting = self.get_meeting(id)?;
        Ok(self.fs.get_artifact_path(&meeting.dir_path, kind))
    }

    pub fn has_artifact(&self, id: &str, kind: &ArtifactKind) -> Result<bool> {
        let meeting = self.get_meeting(id)?;
        Ok(self.fs.has_artifact(&meeting.dir_path, kind))
    }

    pub fn read_artifact(&self, id: &str, kind: &ArtifactKind) -> Result<Vec<u8>> {
        let meeting = self.get_meeting(id)?;
        Ok(self.fs.read_artifact(&meeting.dir_path, kind)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup() -> (Library, TempDir) {
        let tmp = TempDir::new().unwrap();
        let lib = Library::new_in_memory(tmp.path().to_path_buf()).unwrap();
        (lib, tmp)
    }

    #[test]
    fn test_prepare_and_finalize_meeting() {
        let (lib, _tmp) = setup();

        let prepared = lib.prepare_meeting().unwrap();
        assert!(!prepared.id.is_empty());
        assert!(prepared.dir_path.exists());

        let info = RecordingInfo {
            duration_secs: 1800.0,
            has_video: true,
            file_size: 512 * 1024,
            tracks: vec![
                TrackInfo { index: 0, label: "mic".to_string(), codec: "opus".to_string() },
                TrackInfo { index: 1, label: "system".to_string(), codec: "opus".to_string() },
            ],
        };

        let meeting = lib.finalize_meeting(&prepared.id, info).unwrap();
        assert_eq!(meeting.id, prepared.id);
        assert!(meeting.has_video);
        assert_eq!(meeting.media_status, MediaStatus::Present);
    }

    #[test]
    fn test_list_meetings() {
        let (lib, _tmp) = setup();

        let p1 = lib.prepare_meeting().unwrap();
        lib.finalize_meeting(&p1.id, RecordingInfo {
            duration_secs: 100.0, has_video: false, file_size: 1024,
            tracks: vec![],
        }).unwrap();

        let p2 = lib.prepare_meeting().unwrap();
        lib.finalize_meeting(&p2.id, RecordingInfo {
            duration_secs: 200.0, has_video: true, file_size: 2048,
            tracks: vec![],
        }).unwrap();

        let list = lib.list_meetings().unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_delete_everything() {
        let (lib, _tmp) = setup();
        let p = lib.prepare_meeting().unwrap();
        lib.finalize_meeting(&p.id, RecordingInfo {
            duration_secs: 60.0, has_video: false, file_size: 512,
            tracks: vec![],
        }).unwrap();

        lib.delete_meeting(&p.id, DeleteMode::Everything).unwrap();
        assert!(lib.get_meeting(&p.id).is_err());
        assert!(!p.dir_path.exists());
    }

    #[test]
    fn test_delete_media_only() {
        let (lib, _tmp) = setup();
        let p = lib.prepare_meeting().unwrap();

        lib.finalize_meeting(&p.id, RecordingInfo {
            duration_secs: 60.0, has_video: true, file_size: 512,
            tracks: vec![],
        }).unwrap();

        lib.save_artifact(&p.id, &ArtifactKind::Recording, b"video").unwrap();
        lib.save_artifact(&p.id, &ArtifactKind::TranscriptTxt, b"text").unwrap();

        lib.delete_meeting(&p.id, DeleteMode::MediaOnly).unwrap();

        let m = lib.get_meeting(&p.id).unwrap();
        assert_eq!(m.media_status, MediaStatus::Deleted);
        assert!(!lib.has_artifact(&p.id, &ArtifactKind::Recording).unwrap());
        assert!(lib.has_artifact(&p.id, &ArtifactKind::TranscriptTxt).unwrap());
    }

    #[test]
    fn test_artifacts() {
        let (lib, _tmp) = setup();
        let p = lib.prepare_meeting().unwrap();

        lib.finalize_meeting(&p.id, RecordingInfo {
            duration_secs: 60.0, has_video: false, file_size: 512,
            tracks: vec![],
        }).unwrap();

        lib.save_artifact(&p.id, &ArtifactKind::TranscriptJson, b"{\"text\":\"hello\"}").unwrap();
        assert!(lib.has_artifact(&p.id, &ArtifactKind::TranscriptJson).unwrap());

        let data = lib.read_artifact(&p.id, &ArtifactKind::TranscriptJson).unwrap();
        assert_eq!(data, b"{\"text\":\"hello\"}");
    }
}
