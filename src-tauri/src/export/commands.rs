use std::path::PathBuf;

use crate::library::api::Library;

use super::types::{AudioFormat, SaveMode, TranscriptFormat, VideoFormat};
use super::ExportError;

/// Tauri command: export audio from a meeting's recording to the specified format.
///
/// Extracts audio from the MKV recording, mixes multi-track to mono when the
/// target format requires it, and encodes to the target format using FFmpeg.
#[tauri::command]
pub fn export_audio(
    id: String,
    format: AudioFormat,
    save_mode: SaveMode,
    library: tauri::State<'_, Library>,
) -> Result<PathBuf, ExportError> {
    let meeting = library
        .get_meeting(&id)
        .map_err(|e| ExportError::Library(e.to_string()))?;

    super::audio::export_audio(&meeting.dir_path, format, &save_mode)
}

/// Tauri command: export video from a meeting's recording to the specified format.
///
/// Transcodes the MKV H.265 recording to the target codec and container
/// combination using FFmpeg.
#[tauri::command]
pub fn export_video(
    id: String,
    format: VideoFormat,
    save_mode: SaveMode,
    library: tauri::State<'_, Library>,
) -> Result<PathBuf, ExportError> {
    let meeting = library
        .get_meeting(&id)
        .map_err(|e| ExportError::Library(e.to_string()))?;

    super::video::export_video(&meeting.dir_path, format, &save_mode)
}

/// Tauri command: export a meeting's transcript to the specified format.
///
/// - TXT: copies existing transcript.txt
/// - JSON: copies existing transcript.json
/// - SRT: generates subtitle file from transcript.json segments
/// - PDF: generates a formatted PDF document from transcript.json
#[tauri::command]
pub fn export_transcript(
    id: String,
    format: TranscriptFormat,
    save_mode: SaveMode,
    library: tauri::State<'_, Library>,
) -> Result<PathBuf, ExportError> {
    let meeting = library
        .get_meeting(&id)
        .map_err(|e| ExportError::Library(e.to_string()))?;

    super::transcript::export_transcript(&meeting.dir_path, format, &save_mode)
}
