use std::path::PathBuf;

use crate::library::api::Library;

use super::types::{AudioFormat, SaveMode, TranscriptFormat, VideoFormat};
use super::ExportError;

/// Tauri command: export audio from a meeting's recording to the specified format.
///
/// Extracts audio from the MKV recording, mixes multi-track to stereo when the
/// target format requires it, and encodes to the target format using FFmpeg.
/// Runs FFmpeg on a blocking thread to avoid stalling the async runtime.
#[tauri::command]
pub async fn export_audio(
    id: String,
    format: AudioFormat,
    save_mode: SaveMode,
    library: tauri::State<'_, Library>,
) -> Result<PathBuf, ExportError> {
    let meeting = library
        .get_meeting(&id)
        .map_err(|e| ExportError::Library(e.to_string()))?;

    let dir_path = meeting.dir_path.clone();
    tokio::task::spawn_blocking(move || {
        super::audio::export_audio(&dir_path, format, &save_mode)
    })
    .await
    .map_err(|e| ExportError::Library(format!("Task join error: {}", e)))?
}

/// Tauri command: export video from a meeting's recording to the specified format.
///
/// Transcodes the MKV H.265 recording to the target codec and container
/// combination using FFmpeg. Runs on a blocking thread.
#[tauri::command]
pub async fn export_video(
    id: String,
    format: VideoFormat,
    save_mode: SaveMode,
    library: tauri::State<'_, Library>,
) -> Result<PathBuf, ExportError> {
    let meeting = library
        .get_meeting(&id)
        .map_err(|e| ExportError::Library(e.to_string()))?;

    let dir_path = meeting.dir_path.clone();
    tokio::task::spawn_blocking(move || {
        super::video::export_video(&dir_path, format, &save_mode)
    })
    .await
    .map_err(|e| ExportError::Library(format!("Task join error: {}", e)))?
}

/// Tauri command: export a meeting's transcript to the specified format.
///
/// - TXT: copies existing transcript.txt
/// - JSON: copies existing transcript.json
/// - SRT: generates subtitle file from transcript.json segments
/// - PDF: generates a formatted PDF document from transcript.json
///
/// Runs on a blocking thread for PDF generation and file I/O.
#[tauri::command]
pub async fn export_transcript(
    id: String,
    format: TranscriptFormat,
    save_mode: SaveMode,
    library: tauri::State<'_, Library>,
) -> Result<PathBuf, ExportError> {
    let meeting = library
        .get_meeting(&id)
        .map_err(|e| ExportError::Library(e.to_string()))?;

    let dir_path = meeting.dir_path.clone();
    tokio::task::spawn_blocking(move || {
        super::transcript::export_transcript(&dir_path, format, &save_mode)
    })
    .await
    .map_err(|e| ExportError::Library(format!("Task join error: {}", e)))?
}
