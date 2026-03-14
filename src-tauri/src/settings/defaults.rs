use std::path::PathBuf;

use super::config::{
    AppSettings, AudioSettings, GeneralSettings, RagSettings, TranscriptionSettings, VideoSettings,
};

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            audio: AudioSettings::default(),
            video: VideoSettings::default(),
            transcription: TranscriptionSettings::default(),
            rag: RagSettings::default(),
        }
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        let recordings_dir = dirs::home_dir()
            .unwrap_or_else(|| {
                let fallback = std::env::var("HOME")
                    .unwrap_or_else(|_| "/tmp/hlusra".to_string());
                eprintln!(
                    "WARNING: could not determine home dir, falling back to {}",
                    fallback
                );
                PathBuf::from(fallback)
            })
            .join("Hlusra")
            .join("recordings");

        Self {
            recordings_dir: recordings_dir.to_string_lossy().to_string(),
            auto_meeting_name: "Reunião {date} {time}".to_string(),
            start_minimized: false,
        }
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            codec: "opus".to_string(),
            bitrate: 64000,
        }
    }
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            codec: "h265".to_string(),
            backend: "vaapi".to_string(),
            container: "mkv".to_string(),
            bitrate: 2_000_000,
            fps: 15,
            resolution: "720p".to_string(),
        }
    }
}

impl Default for TranscriptionSettings {
    fn default() -> Self {
        Self {
            provider: "local".to_string(),
            api_url: String::new(),
            api_key: String::new(),
            model: "base".to_string(),
        }
    }
}

impl Default for RagSettings {
    fn default() -> Self {
        Self {
            embeddings_url: String::new(),
            embeddings_api_key: String::new(),
            embeddings_model: String::new(),
            chat_url: String::new(),
            chat_api_key: String::new(),
            chat_model: String::new(),
            chunk_size: 500,
            top_k: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults_are_sensible() {
        let settings = AppSettings::default();
        assert!(settings.general.recordings_dir.contains("Hlusra"));
        assert_eq!(settings.video.codec, "h265");
        assert_eq!(settings.video.backend, "vaapi");
        assert_eq!(settings.video.fps, 15);
        assert_eq!(settings.video.resolution, "720p");
        assert_eq!(settings.video.bitrate, 2_000_000);
        assert_eq!(settings.audio.bitrate, 64000);
        assert_eq!(settings.rag.chunk_size, 500);
        assert_eq!(settings.rag.top_k, 5);
        assert_eq!(settings.transcription.provider, "local");
        assert_eq!(settings.transcription.model, "base");
        assert!(!settings.general.start_minimized);
    }
}
