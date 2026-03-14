use tauri::State;

use crate::library::Library;
use crate::library::types::{ArtifactKind, TranscriptionStatus};
use crate::settings::config::load_settings;
use crate::transcription::api::ApiProvider;
use crate::transcription::local::LocalProvider;
use crate::transcription::models;
use crate::transcription::orchestrator::{self, TranscriptionInput};
use crate::transcription::provider::TranscriptionProvider;
use crate::transcription::types::WhisperModel;

// ---------------------------------------------------------------------------
// Provider selection
// ---------------------------------------------------------------------------

/// Creates the appropriate transcription provider based on the user's settings.
fn create_provider() -> Result<Box<dyn TranscriptionProvider>, String> {
    let settings = load_settings().map_err(|e| format!("Failed to load settings: {e}"))?;

    match settings.transcription.provider.as_str() {
        "api" => {
            let url = settings.transcription.api_url;
            let key = settings.transcription.api_key;
            let model = settings.transcription.model;
            Ok(Box::new(ApiProvider::new(url, key, model)))
        }
        _ => {
            // "local" or any unrecognised value falls back to local.
            let active_model = models::get_active_model()?;
            Ok(Box::new(LocalProvider::new(active_model)))
        }
    }
}

// ---------------------------------------------------------------------------
// Transcription commands
// ---------------------------------------------------------------------------

/// Transcribes a meeting using the provider selected in settings.
///
/// Locks on the library are held only briefly for status updates and file I/O.
/// The heavy FFmpeg extraction and Whisper inference run on a blocking thread.
#[tauri::command]
pub async fn transcribe_meeting(
    id: String,
    library: State<'_, Library>,
) -> Result<(), String> {
    let provider = create_provider()?;

    // --- brief lock: read meeting data & mark as processing ----------------
    let input = {
        let meeting = library
            .get_meeting(&id)
            .map_err(|e| format!("Meeting not found: {e}"))?;

        let mkv_path = library.get_artifact_path(&id, &ArtifactKind::Recording)
            .map_err(|e| format!("Failed to get recording path: {e}"))?;
        if !mkv_path.exists() {
            return Err(format!(
                "Recording file does not exist: {}",
                mkv_path.display()
            ));
        }

        library
            .update_transcription_status(&id, TranscriptionStatus::Processing)
            .map_err(|e| format!("DB error (set processing): {e}"))?;

        TranscriptionInput {
            meeting_dir: meeting.dir_path.clone(),
            mkv_path,
        }
    };
    // --- lock released -----------------------------------------------------

    // Run the heavy work on a blocking thread.
    let result = tokio::task::spawn_blocking(move || {
        orchestrator::run_transcription_pipeline(&input, provider.as_ref())
    })
    .await
    .map_err(|e| format!("Transcription task panicked: {e}"))?;

    // --- brief lock: persist artifacts & update status ---------------------
    match result {
        Ok(output) => {
            library
                .save_artifact(
                    &id,
                    &ArtifactKind::TranscriptJson,
                    &output.json_bytes,
                )
                .map_err(|e| format!("Failed to save transcript.json: {e}"))?;

            library
                .save_artifact(
                    &id,
                    &ArtifactKind::TranscriptTxt,
                    &output.txt_bytes,
                )
                .map_err(|e| format!("Failed to save transcript.txt: {e}"))?;

            library
                .update_transcription_status(&id, TranscriptionStatus::Done)
                .map_err(|e| format!("DB error (set done): {e}"))?;

            Ok(())
        }
        Err(err) => {
            let _ = library.update_transcription_status(&id, TranscriptionStatus::Failed);
            Err(err)
        }
    }
}

/// Re-transcribes a meeting (resets status, then runs transcription again).
#[tauri::command]
pub async fn retranscribe_meeting(
    id: String,
    library: State<'_, Library>,
) -> Result<(), String> {
    library
        .update_transcription_status(&id, TranscriptionStatus::Pending)
        .map_err(|e| format!("DB error: {e}"))?;

    transcribe_meeting(id, library).await
}

/// Returns the current transcription status for a meeting.
#[tauri::command]
pub fn get_transcription_status(
    id: String,
    library: State<'_, Library>,
) -> Result<TranscriptionStatus, String> {
    let meeting = library
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
