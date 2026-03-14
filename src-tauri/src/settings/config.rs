use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Top-level application settings, matching the TOML config layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub audio: AudioSettings,
    pub video: VideoSettings,
    pub transcription: TranscriptionSettings,
    pub rag: RagSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub recordings_dir: String,
    pub auto_meeting_name: String,
    pub start_minimized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub codec: String,
    pub bitrate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSettings {
    pub codec: String,
    pub backend: String,
    pub container: String,
    pub bitrate: u32,
    pub fps: u32,
    pub resolution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSettings {
    pub provider: String,
    pub api_url: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagSettings {
    pub embeddings_url: String,
    pub embeddings_api_key: String,
    pub embeddings_model: String,
    pub chat_url: String,
    pub chat_api_key: String,
    pub chat_model: String,
    pub chunk_size: u32,
    pub top_k: u32,
}

/// Error type for settings operations.
#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML deserialization error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),
    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
}

impl serde::Serialize for SettingsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Returns the path to the config file: `~/.config/hlusra/config.toml`
pub fn config_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("~"))
                .join(".config")
        })
        .join("hlusra");
    config_dir.join("config.toml")
}

/// Load settings from disk. If the config file does not exist, creates it with
/// defaults and returns those defaults.
pub fn load_settings() -> Result<AppSettings, SettingsError> {
    let path = config_path();

    if !path.exists() {
        let defaults = AppSettings::default();
        save_settings(&defaults)?;
        return Ok(defaults);
    }

    let content = fs::read_to_string(&path)?;
    let settings: AppSettings = toml::from_str(&content)?;
    Ok(settings)
}

/// Save settings to disk, creating parent directories if needed.
pub fn save_settings(settings: &AppSettings) -> Result<(), SettingsError> {
    let path = config_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(settings)?;
    fs::write(&path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_roundtrip_toml() {
        let settings = AppSettings::default();
        let toml_str = toml::to_string_pretty(&settings).unwrap();
        let parsed: AppSettings = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.general.recordings_dir, settings.general.recordings_dir);
        assert_eq!(parsed.video.codec, settings.video.codec);
        assert_eq!(parsed.rag.chunk_size, settings.rag.chunk_size);
    }

    #[test]
    fn test_save_and_load_to_custom_path() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("config.toml");

        let settings = AppSettings::default();
        let content = toml::to_string_pretty(&settings).unwrap();
        std::fs::write(&path, &content).unwrap();

        let loaded_content = std::fs::read_to_string(&path).unwrap();
        let loaded: AppSettings = toml::from_str(&loaded_content).unwrap();
        assert_eq!(loaded.video.fps, 15);
        assert_eq!(loaded.audio.codec, "opus");
    }
}
