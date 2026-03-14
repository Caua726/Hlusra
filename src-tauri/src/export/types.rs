use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Supported audio export formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioFormat {
    Mp3,
    Wav,
    Opus,
    Ogg,
}

impl AudioFormat {
    /// File extension (without leading dot).
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Mp3 => "mp3",
            Self::Wav => "wav",
            Self::Opus => "opus",
            Self::Ogg => "ogg",
        }
    }

    /// Whether this format requires mixing multiple tracks into a single mono/stereo stream.
    pub fn requires_mixdown(&self) -> bool {
        match self {
            Self::Mp3 | Self::Wav | Self::Ogg => true,
            Self::Opus => false,
        }
    }
}

/// Supported video export formats (container + codec combinations).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoFormat {
    Mp4H264,
    Mp4H265,
    MkvH264,
    MkvH265,
}

impl VideoFormat {
    /// File extension (without leading dot).
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Mp4H264 | Self::Mp4H265 => "mp4",
            Self::MkvH264 | Self::MkvH265 => "mkv",
        }
    }

    /// FFmpeg codec name.
    pub fn codec_name(&self) -> &'static str {
        match self {
            Self::Mp4H264 | Self::MkvH264 => "libx264",
            Self::Mp4H265 | Self::MkvH265 => "libx265",
        }
    }

    /// Container format name for FFmpeg.
    pub fn container_name(&self) -> &'static str {
        match self {
            Self::Mp4H264 | Self::Mp4H265 => "mp4",
            Self::MkvH264 | Self::MkvH265 => "matroska",
        }
    }
}

/// Supported transcript export formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptFormat {
    Txt,
    Json,
    Srt,
    Pdf,
}

impl TranscriptFormat {
    /// File extension (without leading dot).
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Txt => "txt",
            Self::Json => "json",
            Self::Srt => "srt",
            Self::Pdf => "pdf",
        }
    }
}

/// Controls where the exported file is saved.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "mode")]
pub enum SaveMode {
    /// Save directly into the meeting directory.
    Save,
    /// Save to a user-chosen path.
    SaveAs { path: PathBuf },
}

/// Resolve the final output path based on the save mode.
///
/// In `Save` mode the file is placed inside `meeting_dir`.
/// In `SaveAs` mode the caller-supplied path is used verbatim.
pub fn resolve_output_path(meeting_dir: &Path, filename: &str, save_mode: &SaveMode) -> PathBuf {
    match save_mode {
        SaveMode::Save => meeting_dir.join(filename),
        SaveMode::SaveAs { path } => path.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_format_extensions() {
        assert_eq!(AudioFormat::Mp3.extension(), "mp3");
        assert_eq!(AudioFormat::Wav.extension(), "wav");
        assert_eq!(AudioFormat::Opus.extension(), "opus");
        assert_eq!(AudioFormat::Ogg.extension(), "ogg");
    }

    #[test]
    fn test_audio_mixdown_requirements() {
        assert!(AudioFormat::Mp3.requires_mixdown());
        assert!(AudioFormat::Wav.requires_mixdown());
        assert!(AudioFormat::Ogg.requires_mixdown());
        assert!(!AudioFormat::Opus.requires_mixdown());
    }

    #[test]
    fn test_video_format_properties() {
        assert_eq!(VideoFormat::Mp4H264.extension(), "mp4");
        assert_eq!(VideoFormat::Mp4H264.codec_name(), "libx264");
        assert_eq!(VideoFormat::MkvH265.extension(), "mkv");
        assert_eq!(VideoFormat::MkvH265.codec_name(), "libx265");
    }

    #[test]
    fn test_transcript_format_extensions() {
        assert_eq!(TranscriptFormat::Txt.extension(), "txt");
        assert_eq!(TranscriptFormat::Json.extension(), "json");
        assert_eq!(TranscriptFormat::Srt.extension(), "srt");
        assert_eq!(TranscriptFormat::Pdf.extension(), "pdf");
    }

    #[test]
    fn test_resolve_output_save() {
        let dir = PathBuf::from("/meetings/abc123");
        let path = resolve_output_path(&dir, "audio.mp3", &SaveMode::Save);
        assert_eq!(path, PathBuf::from("/meetings/abc123/audio.mp3"));
    }

    #[test]
    fn test_resolve_output_save_as() {
        let dir = PathBuf::from("/meetings/abc123");
        let save_as = SaveMode::SaveAs {
            path: PathBuf::from("/home/user/export.wav"),
        };
        let path = resolve_output_path(&dir, "audio.wav", &save_as);
        assert_eq!(path, PathBuf::from("/home/user/export.wav"));
    }

    #[test]
    fn test_save_mode_serde() {
        let save = SaveMode::Save;
        let json = serde_json::to_string(&save).unwrap();
        assert!(json.contains("save"));

        let save_as = SaveMode::SaveAs {
            path: PathBuf::from("/tmp/export.mp3"),
        };
        let json = serde_json::to_string(&save_as).unwrap();
        assert!(json.contains("/tmp/export.mp3"));
    }
}
