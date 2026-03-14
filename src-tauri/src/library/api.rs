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

        // Track prepared meeting internally
        self.prepared.lock().unwrap().insert(id.clone(), dir_path.clone());

        Ok(PreparedMeeting { id, dir_path })
    }

    /// Library tracks prepared meetings internally to avoid redundant dir_path param
    pub fn finalize_meeting(&self, id: &str, info: RecordingInfo) -> Result<Meeting> {
        let now = Utc::now();
        let title = format!("Reunião {}", now.format("%Y-%m-%d %H:%M"));

        let dir_path = self.prepared.lock().unwrap().remove(id)
            .ok_or(LibraryError::NotFound(format!("No prepared meeting with id {}", id)))?;

        let meeting = Meeting {
            id: id.to_string(),
            title,
            created_at: now,
            duration_secs: info.duration_secs,
            has_video: info.has_video,
            file_size: info.file_size,
            dir_path,
            media_status: MediaStatus::Present,
            transcription_status: TranscriptionStatus::Pending,
            chat_status: ChatStatus::NotIndexed,
        };

        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        db.insert_meeting(&meeting)?;

        Ok(meeting)
    }

    pub fn get_meeting(&self, id: &str) -> Result<Meeting> {
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        db.get_meeting(id).map_err(|_| LibraryError::NotFound(id.to_string()))
    }

    pub fn list_meetings(&self) -> Result<Vec<MeetingSummary>> {
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        Ok(db.list_meetings()?)
    }

    pub fn update_title(&self, id: &str, title: &str) -> Result<()> {
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        Ok(db.update_title(id, title)?)
    }

    pub fn delete_meeting(&self, id: &str, mode: DeleteMode) -> Result<()> {
        // Single lock acquisition to avoid TOCTOU race
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        let meeting = db.get_meeting(id).map_err(|_| LibraryError::NotFound(id.to_string()))?;

        match mode {
            DeleteMode::Everything => {
                self.fs.delete_meeting_dir(&meeting.dir_path)?;
                db.delete_meeting(id)?;
            }
            DeleteMode::MediaOnly => {
                self.fs.delete_media_files(&meeting.dir_path)?;
                db.update_media_status(id, MediaStatus::Deleted)?;
            }
        }
        Ok(())
    }

    pub fn update_transcription_status(&self, id: &str, status: TranscriptionStatus) -> Result<()> {
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        Ok(db.update_transcription_status(id, status)?)
    }

    pub fn update_chat_status(&self, id: &str, status: ChatStatus) -> Result<()> {
        let db = self.db.lock().map_err(|_| LibraryError::NotFound("lock poisoned".to_string()))?;
        Ok(db.update_chat_status(id, status)?)
    }

    pub fn save_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind, data: &[u8]) -> Result<PathBuf> {
        Ok(self.fs.save_artifact(meeting_dir, kind, data)?)
    }

    pub fn get_artifact_path(&self, meeting_dir: &Path, kind: &ArtifactKind) -> PathBuf {
        self.fs.get_artifact_path(meeting_dir, kind)
    }

    pub fn has_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> bool {
        self.fs.has_artifact(meeting_dir, kind)
    }

    pub fn read_artifact(&self, meeting_dir: &Path, kind: &ArtifactKind) -> Result<Vec<u8>> {
        Ok(self.fs.read_artifact(meeting_dir, kind)?)
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
        lib.save_artifact(&p.dir_path, &ArtifactKind::Recording, b"video").unwrap();
        lib.save_artifact(&p.dir_path, &ArtifactKind::TranscriptTxt, b"text").unwrap();

        lib.finalize_meeting(&p.id, RecordingInfo {
            duration_secs: 60.0, has_video: true, file_size: 512,
            tracks: vec![],
        }).unwrap();

        lib.delete_meeting(&p.id, DeleteMode::MediaOnly).unwrap();

        let m = lib.get_meeting(&p.id).unwrap();
        assert_eq!(m.media_status, MediaStatus::Deleted);
        assert!(!lib.has_artifact(&p.dir_path, &ArtifactKind::Recording));
        assert!(lib.has_artifact(&p.dir_path, &ArtifactKind::TranscriptTxt));
    }

    #[test]
    fn test_artifacts() {
        let (lib, _tmp) = setup();
        let p = lib.prepare_meeting().unwrap();

        lib.save_artifact(&p.dir_path, &ArtifactKind::TranscriptJson, b"{\"text\":\"hello\"}").unwrap();
        assert!(lib.has_artifact(&p.dir_path, &ArtifactKind::TranscriptJson));

        let data = lib.read_artifact(&p.dir_path, &ArtifactKind::TranscriptJson).unwrap();
        assert_eq!(data, b"{\"text\":\"hello\"}");
    }
}
