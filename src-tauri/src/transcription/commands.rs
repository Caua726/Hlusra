use std::path::PathBuf;
use std::sync::Mutex;

use tauri::State;

use crate::library::db::LibraryDb;
use crate::library::fs::LibraryFs;
use crate::library::types::TranscriptionStatus;
use crate::transcription::local::LocalProvider;
use crate::transcription::models;
use crate::transcription::orchestrator;
use crate::transcription::types::WhisperModel;

/// Shared application state that holds library handles.
///
/// This mirrors the shape that `lib.rs` will manage; commands receive it via
/// Tauri's managed state.
pub struct AppState {
    pub db: Mutex<LibraryDb>,
    pub fs: LibraryFs,
}

// ---------------------------------------------------------------------------
// Transcription commands
// ---------------------------------------------------------------------------

/// Transcribes a meeting using the currently active local Whisper model.
#[tauri::command]
pub async fn transcribe_meeting(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let active_model = models::get_active_model()?;
    let provider = LocalProvider::new(active_model);

    let db = state.db.lock().map_err(|e| format!("Lock error: {e}"))?;
    orchestrator::transcribe_meeting(&db, &state.fs, &provider, &id)?;
    Ok(())
}

/// Re-transcribes a meeting (resets status, then runs transcription again).
#[tauri::command]
pub async fn retranscribe_meeting(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    {
        let db = state.db.lock().map_err(|e| format!("Lock error: {e}"))?;
        db.update_transcription_status(&id, TranscriptionStatus::Pending)
            .map_err(|e| format!("DB error: {e}"))?;
    }

    transcribe_meeting(id, state).await
}

/// Returns the current transcription status for a meeting.
#[tauri::command]
pub fn get_transcription_status(
    id: String,
    state: State<'_, AppState>,
) -> Result<TranscriptionStatus, String> {
    let db = state.db.lock().map_err(|e| format!("Lock error: {e}"))?;
    let meeting = db
        .get_meeting(&id)
        .map_err(|e| format!("Meeting not found: {e}"))?;
    Ok(meeting.transcription_status)
}

// ---------------------------------------------------------------------------
// Model management commands
// ---------------------------------------------------------------------------

/// Lists all known Whisper models, marking which are already downloaded.
#[tauri::command]
pub fn list_available_models() -> Result<Vec<WhisperModel>, String> {
    models::list_available_models()
}

/// Lists only the models that have been downloaded to disk.
#[tauri::command]
pub fn get_downloaded_models() -> Result<Vec<WhisperModel>, String> {
    models::get_downloaded_models()
}

/// Downloads a model by name (e.g. "tiny", "base", "small", "medium", "large").
#[tauri::command]
pub async fn download_model(model: String) -> Result<(), String> {
    // Run the (potentially slow) download on a blocking thread so we don't
    // block the Tauri async runtime.
    tokio::task::spawn_blocking(move || models::download_model(&model))
        .await
        .map_err(|e| format!("Download task panicked: {e}"))?
}

/// Returns the currently active Whisper model.
#[tauri::command]
pub fn get_active_model() -> Result<WhisperModel, String> {
    models::get_active_model()
}

/// Sets the active Whisper model. The model must already be downloaded.
#[tauri::command]
pub fn set_active_model(model: String) -> Result<(), String> {
    models::set_active_model(&model)
}
