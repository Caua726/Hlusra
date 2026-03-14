use super::config::{self, AppSettings, SettingsError};

/// Tauri command: returns the current app settings, loading from disk
/// (or creating defaults if the config file is missing).
#[tauri::command]
pub fn get_settings() -> Result<AppSettings, SettingsError> {
    config::load_settings()
}

/// Tauri command: persists the provided settings to disk.
#[tauri::command]
pub fn update_settings(settings: AppSettings) -> Result<(), SettingsError> {
    config::save_settings(&settings)
}
