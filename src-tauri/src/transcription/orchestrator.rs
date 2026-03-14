use std::path::{Path, PathBuf};

use crate::library::db::LibraryDb;
use crate::library::fs::LibraryFs;
use crate::library::types::{ArtifactKind, TranscriptionStatus};
use crate::transcription::provider::TranscriptionProvider;
use crate::transcription::types::TranscriptResult;

/// Orchestrates the full transcription pipeline for a meeting:
///
/// 1. Looks up the meeting in the library database.
/// 2. Extracts the mic track (stream 0) from the MKV recording via FFmpeg CLI
///    and writes it to a temporary 16 kHz mono PCM s16le WAV file.
/// 3. Sends the WAV to the active `TranscriptionProvider`.
/// 4. Saves `transcript.json` and `transcript.txt` via `LibraryFs`.
/// 5. Updates `transcription_status` in the database.
/// 6. Cleans up the temporary WAV file.
pub fn transcribe_meeting(
    db: &LibraryDb,
    fs: &LibraryFs,
    provider: &dyn TranscriptionProvider,
    meeting_id: &str,
) -> Result<TranscriptResult, String> {
    // Mark as processing.
    db.update_transcription_status(meeting_id, TranscriptionStatus::Processing)
        .map_err(|e| format!("DB error (set processing): {e}"))?;

    let result = run_pipeline(db, fs, provider, meeting_id);

    match &result {
        Ok(_) => {
            db.update_transcription_status(meeting_id, TranscriptionStatus::Done)
                .map_err(|e| format!("DB error (set done): {e}"))?;
        }
        Err(_) => {
            let _ = db.update_transcription_status(meeting_id, TranscriptionStatus::Failed);
        }
    }

    result
}

/// Inner pipeline that may fail at any step. The caller handles status updates.
fn run_pipeline(
    db: &LibraryDb,
    lib_fs: &LibraryFs,
    provider: &dyn TranscriptionProvider,
    meeting_id: &str,
) -> Result<TranscriptResult, String> {
    // 1. Look up the meeting.
    let meeting = db
        .get_meeting(meeting_id)
        .map_err(|e| format!("Meeting not found: {e}"))?;

    let mkv_path = lib_fs.get_artifact_path(&meeting.dir_path, &ArtifactKind::Recording);
    if !mkv_path.exists() {
        return Err(format!(
            "Recording file does not exist: {}",
            mkv_path.display()
        ));
    }

    // 2. Extract mic track to temporary WAV.
    let temp_wav = meeting.dir_path.join("_temp_mic.wav");
    extract_mic_to_wav(&mkv_path, &temp_wav)?;

    // 3. Run transcription.
    let result = provider.transcribe(&temp_wav);

    // 6. Clean up temp WAV regardless of success/failure.
    let _ = std::fs::remove_file(&temp_wav);

    let transcript = result?;

    // 4. Save artifacts via the library filesystem.
    let json_bytes = serde_json::to_vec_pretty(&transcript)
        .map_err(|e| format!("Failed to serialize transcript JSON: {e}"))?;
    lib_fs
        .save_artifact(&meeting.dir_path, &ArtifactKind::TranscriptJson, &json_bytes)
        .map_err(|e| format!("Failed to save transcript.json: {e}"))?;

    lib_fs
        .save_artifact(
            &meeting.dir_path,
            &ArtifactKind::TranscriptTxt,
            transcript.full_text.as_bytes(),
        )
        .map_err(|e| format!("Failed to save transcript.txt: {e}"))?;

    Ok(transcript)
}

/// Extracts the mic track (stream 0) from the MKV recording using FFmpeg CLI
/// and writes it to a 16 kHz mono PCM s16le WAV file.
fn extract_mic_to_wav(mkv_path: &Path, wav_path: &Path) -> Result<(), String> {
    let output = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(mkv_path)
        .arg("-map")
        .arg("0:a:0") // first audio stream (mic)
        .arg("-ac")
        .arg("1") // mono
        .arg("-ar")
        .arg("16000") // 16 kHz
        .arg("-codec:a")
        .arg("pcm_s16le")
        .arg(wav_path)
        .output()
        .map_err(|e| format!("Failed to run ffmpeg: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FFmpeg extraction failed: {stderr}"));
    }

    Ok(())
}
