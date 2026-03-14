use std::fs;
use std::io;
use std::path::PathBuf;

use crate::transcription::types::{all_models, WhisperModel};

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
pub fn download_model(model_name: &str) -> Result<(), String> {
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

    let mut response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to start download for {model_name}: {e}"))?;

    if !response.status().is_success() {
        return Err(format!(
            "Download failed for {model_name}: HTTP {}",
            response.status()
        ));
    }

    let mut file = fs::File::create(&part_path)
        .map_err(|e| format!("Failed to create temp file: {e}"))?;
    io::copy(&mut response, &mut file)
        .map_err(|e| format!("Failed to download {model_name}: {e}"))?;

    fs::rename(&part_path, &dest)
        .map_err(|e| format!("Failed to finalize model file: {e}"))?;

    Ok(())
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

    let mut models = list_available_models()?;
    models
        .into_iter()
        .find(|m| m.name == name)
        .ok_or_else(|| format!("Active model '{name}' is not in the catalogue"))
}

/// Persists the user's model selection. The model must already be downloaded.
pub fn set_active_model(model_name: &str) -> Result<(), String> {
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
