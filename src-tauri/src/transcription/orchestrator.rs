use std::path::{Path, PathBuf};

use crate::transcription::provider::TranscriptionProvider;
use crate::transcription::types::TranscriptResult;

/// Input data the orchestrator needs to run the transcription pipeline.
/// Gathered by the caller *before* spawning the blocking work, so no locks
/// are held during the heavy FFmpeg + Whisper stage.
pub struct TranscriptionInput {
    /// Path to the meeting directory (contains recording.mkv).
    pub meeting_dir: PathBuf,
    /// Full path to the MKV recording file.
    pub mkv_path: PathBuf,
}

/// Output produced by the blocking pipeline, ready to be persisted by the
/// caller while briefly re-acquiring any necessary locks.
pub struct TranscriptionOutput {
    pub transcript: TranscriptResult,
    pub json_bytes: Vec<u8>,
    pub txt_bytes: Vec<u8>,
}

/// Runs the CPU-intensive part of the transcription pipeline:
///
/// 1. Extracts the mic track from the MKV via FFmpeg CLI into a temp WAV.
/// 2. Sends the WAV to the `TranscriptionProvider`.
/// 3. Serializes the result into JSON and plain text bytes.
/// 4. Cleans up the temporary WAV file.
///
/// This function is designed to run inside `tokio::task::spawn_blocking` and
/// does **not** touch the database or any shared state.
pub fn run_transcription_pipeline(
    input: &TranscriptionInput,
    provider: &dyn TranscriptionProvider,
) -> Result<TranscriptionOutput, String> {
    // 1. Extract mic track to temporary WAV.
    let temp_wav = input.meeting_dir.join("_temp_mic.wav");
    extract_mic_to_wav(&input.mkv_path, &temp_wav)?;

    // 2. Run transcription.
    let result = provider.transcribe(&temp_wav);

    // Clean up temp WAV regardless of success/failure.
    let _ = std::fs::remove_file(&temp_wav);

    let transcript = result?;

    // 3. Serialize artifacts.
    let json_bytes = serde_json::to_vec_pretty(&transcript)
        .map_err(|e| format!("Failed to serialize transcript JSON: {e}"))?;
    let txt_bytes = transcript.full_text.as_bytes().to_vec();

    Ok(TranscriptionOutput {
        transcript,
        json_bytes,
        txt_bytes,
    })
}

/// Extracts the mic track (stream 0) from the MKV recording using FFmpeg CLI
/// and writes it to a 16 kHz mono PCM s16le WAV file.
fn extract_mic_to_wav(mkv_path: &Path, wav_path: &Path) -> Result<(), String> {
    if !mkv_path.exists() {
        return Err(format!(
            "Input MKV file does not exist: {}",
            mkv_path.display()
        ));
    }

    let output = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-loglevel")
        .arg("error")
        .arg("-i")
        .arg(mkv_path)
        .arg("-map")
        .arg("0:a:0") // first audio stream (mic)
        .arg("-vn")
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
