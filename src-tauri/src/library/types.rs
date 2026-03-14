use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meeting {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub duration_secs: f64,
    pub has_video: bool,
    pub file_size: u64,
    pub dir_path: PathBuf,
    pub tracks: Vec<TrackInfo>,
    pub media_status: MediaStatus,
    pub transcription_status: TranscriptionStatus,
    pub chat_status: ChatStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingSummary {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub duration_secs: f64,
    pub has_video: bool,
    pub file_size: u64,
    pub media_status: MediaStatus,
    pub transcription_status: TranscriptionStatus,
    pub chat_status: ChatStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeetingDetail {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub duration_secs: f64,
    pub has_video: bool,
    pub file_size: u64,
    pub dir_path: PathBuf,
    pub media_status: MediaStatus,
    pub transcription_status: TranscriptionStatus,
    pub chat_status: ChatStatus,
    pub tracks: Vec<TrackInfo>,
    pub transcript: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackInfo {
    pub index: usize,
    pub label: String,
    pub codec: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedMeeting {
    pub id: String,
    pub dir_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingInfo {
    pub duration_secs: f64,
    pub has_video: bool,
    pub file_size: u64,
    pub tracks: Vec<TrackInfo>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaStatus {
    #[default]
    Present,
    Deleted,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptionStatus {
    #[default]
    Pending,
    Processing,
    Done,
    Failed,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatStatus {
    #[default]
    NotIndexed,
    Indexing,
    Ready,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteMode {
    Everything,
    MediaOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    Recording,
    Thumbnail,
    TranscriptJson,
    TranscriptTxt,
}

impl ArtifactKind {
    pub fn filename(&self) -> &'static str {
        match self {
            Self::Recording => "recording.mkv",
            Self::Thumbnail => "thumbnail.jpg",
            Self::TranscriptJson => "transcript.json",
            Self::TranscriptTxt => "transcript.txt",
        }
    }
}

impl MediaStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Deleted => "deleted",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "present" => Self::Present,
            "deleted" => Self::Deleted,
            other => {
                eprintln!("WARNING: unknown MediaStatus value '{}', defaulting to Present", other);
                Self::Present
            }
        }
    }
}

impl TranscriptionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Done => "done",
            Self::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => Self::Pending,
            "processing" => Self::Processing,
            "done" => Self::Done,
            "failed" => Self::Failed,
            other => {
                eprintln!("WARNING: unknown TranscriptionStatus value '{}', defaulting to Pending", other);
                Self::Pending
            }
        }
    }
}

impl ChatStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotIndexed => "not_indexed",
            Self::Indexing => "indexing",
            Self::Ready => "ready",
            Self::Failed => "failed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "not_indexed" => Self::NotIndexed,
            "indexing" => Self::Indexing,
            "ready" => Self::Ready,
            "failed" => Self::Failed,
            other => {
                eprintln!("WARNING: unknown ChatStatus value '{}', defaulting to NotIndexed", other);
                Self::NotIndexed
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_status_roundtrip() {
        assert_eq!(MediaStatus::from_str(MediaStatus::Present.as_str()), MediaStatus::Present);
        assert_eq!(MediaStatus::from_str(MediaStatus::Deleted.as_str()), MediaStatus::Deleted);
    }

    #[test]
    fn test_artifact_kind_filenames() {
        assert_eq!(ArtifactKind::Recording.filename(), "recording.mkv");
        assert_eq!(ArtifactKind::Thumbnail.filename(), "thumbnail.jpg");
        assert_eq!(ArtifactKind::TranscriptJson.filename(), "transcript.json");
        assert_eq!(ArtifactKind::TranscriptTxt.filename(), "transcript.txt");
    }

    #[test]
    fn test_meeting_serialization() {
        let meeting = Meeting {
            id: "test-id".to_string(),
            title: "Test Meeting".to_string(),
            created_at: Utc::now(),
            duration_secs: 3600.0,
            has_video: true,
            file_size: 1024,
            dir_path: PathBuf::from("/tmp/test"),
            tracks: vec![
                TrackInfo { index: 0, label: "mic".to_string(), codec: "opus".to_string() },
            ],
            media_status: MediaStatus::Present,
            transcription_status: TranscriptionStatus::Pending,
            chat_status: ChatStatus::NotIndexed,
        };
        let json = serde_json::to_string(&meeting).unwrap();
        let deserialized: Meeting = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "test-id");
        assert_eq!(deserialized.tracks.len(), 1);
    }
}
