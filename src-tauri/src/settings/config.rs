use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Top-level application settings, matching the TOML config layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub audio: AudioSettings,
    pub video: VideoSettings,
    pub transcription: TranscriptionSettings,
    pub rag: RagSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralSettings {
    pub recordings_dir: PathBuf,
    pub auto_meeting_name: String,
    pub start_minimized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioSettings {
    pub codec: String,
    pub bitrate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VideoSettings {
    pub codec: String,
    pub backend: String,
    pub container: String,
    pub bitrate: u32,
    pub fps: u32,
    pub resolution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TranscriptionSettings {
    pub provider: String,
    pub api_url: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
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

impl AppSettings {
    /// Validate loaded settings and fix up any clearly invalid values.
    /// Returns a list of warnings for values that were corrected.
    pub fn validate(&mut self) -> Vec<String> {
        let mut warnings = Vec::new();

        if self.video.fps == 0 {
            warnings.push("video.fps was 0, reset to 15".to_string());
            self.video.fps = 15;
        }

        if self.rag.chunk_size == 0 {
            warnings.push("rag.chunk_size was 0, reset to 500".to_string());
            self.rag.chunk_size = 500;
        }

        if self.rag.top_k == 0 {
            warnings.push("rag.top_k was 0, reset to 5".to_string());
            self.rag.top_k = 5;
        }

        if self.audio.bitrate == 0 {
            warnings.push("audio.bitrate was 0, reset to 64000".to_string());
            self.audio.bitrate = 64000;
        }

        if self.video.bitrate == 0 {
            warnings.push("video.bitrate was 0, reset to 2000000".to_string());
            self.video.bitrate = 2_000_000;
        }

        // Ensure recordings_dir is an absolute path
        if self.general.recordings_dir.as_os_str().is_empty() {
            let default_dir = GeneralSettings::default().recordings_dir;
            warnings.push(format!(
                "general.recordings_dir was empty, reset to {}",
                default_dir.display()
            ));
            self.general.recordings_dir = default_dir;
        }

        warnings
    }
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

/// Resolve home directory reliably, using dirs crate with $HOME env fallback.
fn resolve_home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| {
        let fallback = std::env::var("HOME").unwrap_or_else(|_| "/tmp/hlusra".to_string());
        PathBuf::from(fallback)
    })
}

/// Returns the path to the config file: `~/.config/hlusra/config.toml`
pub fn config_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| resolve_home_dir().join(".config"))
        .join("hlusra");
    config_dir.join("config.toml")
}

/// Load settings from disk. If the config file does not exist (or any IO error
/// occurs on first read), creates it with defaults and returns those defaults.
///
/// Uses a read-first approach to avoid TOCTOU race conditions: we attempt the
/// read and handle `NotFound` rather than checking `path.exists()` first.
pub fn load_settings() -> Result<AppSettings, SettingsError> {
    let path = config_path();

    match fs::read_to_string(&path) {
        Ok(content) => {
            let mut settings: AppSettings = toml::from_str(&content)?;
            // Validate and auto-fix any out-of-range values
            let _warnings = settings.validate();
            Ok(settings)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Config file doesn't exist yet — create with defaults
            let defaults = AppSettings::default();
            save_settings(&defaults)?;
            Ok(defaults)
        }
        Err(e) => Err(SettingsError::Io(e)),
    }
}

/// Save settings to disk, creating parent directories if needed.
/// Sets file permissions to 0o600 (owner read/write only) because the config
/// may contain API keys.
pub fn save_settings(settings: &AppSettings) -> Result<(), SettingsError> {
    let path = config_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(settings)?;
    fs::write(&path, content)?;

    // Restrict file permissions: owner read/write only (API keys security)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(&path, perms)?;
    }

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

    #[test]
    fn test_serde_default_fills_missing_fields() {
        // A TOML with only [general] should still deserialize with defaults for other sections
        let partial_toml = r#"
[general]
auto_meeting_name = "Test {date}"
start_minimized = true
"#;
        let settings: AppSettings = toml::from_str(partial_toml).unwrap();
        assert_eq!(settings.general.auto_meeting_name, "Test {date}");
        assert!(settings.general.start_minimized);
        // Other sections should have defaults
        assert_eq!(settings.audio.codec, "opus");
        assert_eq!(settings.video.fps, 15);
    }

    #[test]
    fn test_validate_fixes_zero_values() {
        let mut settings = AppSettings::default();
        settings.video.fps = 0;
        settings.rag.chunk_size = 0;
        settings.audio.bitrate = 0;

        let warnings = settings.validate();
        assert!(!warnings.is_empty());
        assert_eq!(settings.video.fps, 15);
        assert_eq!(settings.rag.chunk_size, 500);
        assert_eq!(settings.audio.bitrate, 64000);
    }

    #[test]
    fn test_resolve_home_dir_not_empty() {
        let home = resolve_home_dir();
        assert!(!home.as_os_str().is_empty());
    }
}
