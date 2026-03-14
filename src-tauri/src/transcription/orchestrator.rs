use std::path::{Path, PathBuf};

use crate::library::db::LibraryDb;
use crate::library::fs::LibraryFs;
use crate::library::types::{ArtifactKind, TranscriptionStatus};
use crate::transcription::provider::TranscriptionProvider;
use crate::transcription::types::TranscriptResult;

/// Orchestrates the full transcription pipeline for a meeting:
///
/// 1. Looks up the meeting in the library database.
/// 2. Extracts the mic track (stream 0) from the MKV recording via `ffmpeg-next`
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

/// Uses `ffmpeg-next` to demux the MKV, select stream 0 (mic), decode, resample
/// to 16 kHz mono, and write a PCM s16le WAV file.
fn extract_mic_to_wav(mkv_path: &Path, wav_path: &Path) -> Result<(), String> {
    ffmpeg_next::init().map_err(|e| format!("FFmpeg init failed: {e}"))?;

    let mut ictx = ffmpeg_next::format::input(mkv_path)
        .map_err(|e| format!("Failed to open MKV: {e}"))?;

    // Find the first audio stream (stream 0 = mic).
    let audio_stream = ictx
        .streams()
        .best(ffmpeg_next::media::Type::Audio)
        .ok_or_else(|| "No audio stream found in MKV".to_string())?;

    let stream_index = audio_stream.index();
    let codec_params = audio_stream.parameters();

    let decoder_codec = ffmpeg_next::codec::context::Context::from_parameters(codec_params)
        .map_err(|e| format!("Failed to create decoder context: {e}"))?;
    let mut decoder = decoder_codec
        .decoder()
        .audio()
        .map_err(|e| format!("Failed to open audio decoder: {e}"))?;

    // Target format.
    let target_rate = 16000u32;
    let target_channel_layout = ffmpeg_next::channel_layout::ChannelLayout::MONO;
    let target_format = ffmpeg_next::format::Sample::I16(ffmpeg_next::format::sample::Type::Packed);

    // Set up resampler.
    let mut resampler = ffmpeg_next::software::resampling::Context::get(
        decoder.format(),
        decoder.channel_layout(),
        decoder.rate(),
        target_format,
        target_channel_layout,
        target_rate,
    )
    .map_err(|e| format!("Failed to create resampler: {e}"))?;

    // Collect all resampled samples into a buffer.
    let mut pcm_data: Vec<i16> = Vec::new();

    let mut decoded_frame = ffmpeg_next::frame::Audio::empty();

    for (stream, packet) in ictx.packets() {
        if stream.index() != stream_index {
            continue;
        }

        decoder
            .send_packet(&packet)
            .map_err(|e| format!("Failed to send packet to decoder: {e}"))?;

        while decoder.receive_frame(&mut decoded_frame).is_ok() {
            let mut resampled = ffmpeg_next::frame::Audio::empty();
            resampler
                .run(&decoded_frame, &mut resampled)
                .map_err(|e| format!("Resampling failed: {e}"))?;

            append_samples(&resampled, &mut pcm_data);
        }
    }

    // Flush decoder.
    decoder
        .send_eof()
        .map_err(|e| format!("Failed to flush decoder: {e}"))?;
    while decoder.receive_frame(&mut decoded_frame).is_ok() {
        let mut resampled = ffmpeg_next::frame::Audio::empty();
        resampler
            .run(&decoded_frame, &mut resampled)
            .map_err(|e| format!("Resampling failed: {e}"))?;
        append_samples(&resampled, &mut pcm_data);
    }

    // Flush resampler (delayed samples).
    {
        let mut flushed = ffmpeg_next::frame::Audio::empty();
        while resampler
            .flush(&mut flushed)
            .map(|_| flushed.samples() > 0)
            .unwrap_or(false)
        {
            append_samples(&flushed, &mut pcm_data);
        }
    }

    // Write WAV file using hound.
    write_wav(wav_path, target_rate, &pcm_data)?;

    Ok(())
}

/// Appends i16 samples from a resampled audio frame to the output buffer.
fn append_samples(frame: &ffmpeg_next::frame::Audio, buf: &mut Vec<i16>) {
    let n_samples = frame.samples();
    if n_samples == 0 {
        return;
    }
    let data = frame.data(0);
    // data is &[u8] representing packed i16 samples.
    let sample_bytes = std::mem::size_of::<i16>();
    let count = data.len() / sample_bytes;
    for i in 0..count.min(n_samples) {
        let offset = i * sample_bytes;
        if offset + sample_bytes <= data.len() {
            let sample = i16::from_le_bytes([data[offset], data[offset + 1]]);
            buf.push(sample);
        }
    }
}

/// Writes raw PCM i16 samples as a WAV file.
fn write_wav(path: &Path, sample_rate: u32, samples: &[i16]) -> Result<(), String> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer =
        hound::WavWriter::create(path, spec).map_err(|e| format!("Failed to create WAV: {e}"))?;

    for &s in samples {
        writer
            .write_sample(s)
            .map_err(|e| format!("Failed to write WAV sample: {e}"))?;
    }

    writer
        .finalize()
        .map_err(|e| format!("Failed to finalize WAV: {e}"))?;

    Ok(())
}
