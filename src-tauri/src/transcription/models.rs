use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use tauri::Emitter;

use crate::transcription::types::{all_models, WhisperModel};

/// Global flag used to signal cancellation of an in-progress model download.
static DOWNLOAD_CANCELLED: AtomicBool = AtomicBool::new(false);

/// Sets the cancellation flag so the next iteration of the download loop aborts.
pub fn cancel_download() {
    DOWNLOAD_CANCELLED.store(true, Ordering::SeqCst);
}

/// Clears the cancellation flag. Called at the start of every download.
fn reset_cancel_flag() {
    DOWNLOAD_CANCELLED.store(false, Ordering::SeqCst);
}

/// Default model name used when no explicit selection has been made.
const DEFAULT_MODEL: &str = "tiny";

/// Name of the file that persists the user's active model selection.
const ACTIVE_MODEL_FILE: &str = ".active_model";

/// Returns the directory where Whisper models are stored.
///
/// Location: `~/.local/share/hlusra/models/`
pub fn models_dir() -> Result<PathBuf, String> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| "Cannot determine XDG data directory".to_string())?;
    let dir = data_dir.join("hlusra").join("models");
    fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create models directory: {e}"))?;
    Ok(dir)
}

/// Lists all known Whisper models, marking which ones have already been downloaded.
pub fn list_available_models() -> Result<Vec<WhisperModel>, String> {
    let dir = models_dir()?;
    let mut models = all_models();
    for m in &mut models {
        m.downloaded = dir.join(m.filename()).exists();
    }
    Ok(models)
}

/// Lists only the models that have been downloaded to disk.
pub fn get_downloaded_models() -> Result<Vec<WhisperModel>, String> {
    Ok(list_available_models()?
        .into_iter()
        .filter(|m| m.downloaded)
        .collect())
}

/// Downloads a model by name from Hugging Face.
///
/// The file is written atomically: we download to a `.part` file first, then rename.
/// Emits `model-download-progress` events to the frontend and checks for
/// cancellation on every chunk.
pub fn download_model(model_name: &str, app: &tauri::AppHandle) -> Result<(), String> {
    reset_cancel_flag();

    let catalogue = all_models();
    let model = catalogue
        .iter()
        .find(|m| m.name == model_name)
        .ok_or_else(|| format!("Unknown model: {model_name}"))?;

    let dir = models_dir()?;
    let dest = dir.join(model.filename());
    if dest.exists() {
        return Ok(()); // already downloaded
    }

    let url = model.download_url();
    let part_path = dir.join(format!("{}.part", model.filename()));

    let do_download = || -> Result<(), String> {
        let mut response = reqwest::blocking::get(&url)
            .map_err(|e| format!("Failed to start download for {model_name}: {e}"))?;

        if !response.status().is_success() {
            return Err(format!(
                "Download failed for {model_name}: HTTP {}",
                response.status()
            ));
        }

        let total = response.content_length().unwrap_or(0);
        let mut file = fs::File::create(&part_path)
            .map_err(|e| format!("Failed to create temp file: {e}"))?;

        let mut downloaded: u64 = 0;
        let mut buf = vec![0u8; 8192];

        loop {
            // Check cancellation flag before reading next chunk.
            if DOWNLOAD_CANCELLED.load(Ordering::SeqCst) {
                return Err("Download cancelled".to_string());
            }

            let n = response
                .read(&mut buf)
                .map_err(|e| format!("Failed to download {model_name}: {e}"))?;
            if n == 0 {
                break;
            }
            file.write_all(&buf[..n])
                .map_err(|e| format!("Failed to write {model_name}: {e}"))?;
            downloaded += n as u64;

            // Emit progress roughly every 100 KB.
            if downloaded % 102_400 < 8192 {
                let _ = app.emit(
                    "model-download-progress",
                    serde_json::json!({
                        "model": model_name,
                        "downloaded": downloaded,
                        "total": total,
                    }),
                );
            }
        }

        file.sync_all()
            .map_err(|e| format!("Failed to sync model file to disk: {e}"))?;

        fs::rename(&part_path, &dest)
            .map_err(|e| format!("Failed to finalize model file: {e}"))?;

        Ok(())
    };

    let result = do_download();
    if result.is_err() {
        let _ = fs::remove_file(&part_path);
    }
    result
}

/// Returns the currently active model. Falls back to the default (`tiny`) if
/// nothing has been explicitly selected, or the persisted choice is invalid.
pub fn get_active_model() -> Result<WhisperModel, String> {
    let dir = models_dir()?;
    let active_file = dir.join(ACTIVE_MODEL_FILE);
    let name = if active_file.exists() {
        fs::read_to_string(&active_file)
            .map_err(|e| format!("Failed to read active model file: {e}"))?
            .trim()
            .to_string()
    } else {
        DEFAULT_MODEL.to_string()
    };

    let models = list_available_models()?;
    models
        .into_iter()
        .find(|m| m.name == name)
        .ok_or_else(|| format!("Active model '{name}' is not in the catalogue"))
}

/// Persists the user's model selection. The model must already be downloaded.
pub fn set_active_model(model_name: &str) -> Result<(), String> {
    let catalogue = all_models();
    if !catalogue.iter().any(|m| m.name == model_name) {
        return Err(format!("Unknown model: '{model_name}'"));
    }

    let dir = models_dir()?;
    let model_file = dir.join(format!("ggml-{model_name}.bin"));
    if !model_file.exists() {
        return Err(format!(
            "Model '{model_name}' has not been downloaded yet"
        ));
    }

    let active_file = dir.join(ACTIVE_MODEL_FILE);
    fs::write(&active_file, model_name)
        .map_err(|e| format!("Failed to persist active model selection: {e}"))?;
    Ok(())
}
