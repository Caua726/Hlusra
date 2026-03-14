use std::path::Path;
use std::process::Command;

/// Generate a video thumbnail by extracting a frame at ~10 seconds.
pub fn generate_video_thumbnail(video_path: &Path, output_path: &Path) -> Result<(), String> {
    let output = Command::new("ffmpeg")
        .args(["-y", "-loglevel", "error"])
        .arg("-i").arg(video_path)
        .args(["-ss", "10", "-vframes", "1", "-vf", "scale=320:-1"])
        .arg(output_path)
        .output()
        .map_err(|e| format!("Failed to run ffmpeg for video thumbnail: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Video thumbnail generation failed: {stderr}"));
    }
    Ok(())
}

/// Generate an audio waveform thumbnail using FFmpeg's showwavespic filter.
/// Produces a 320x80 image with the brand red color.
pub fn generate_audio_waveform(audio_path: &Path, output_path: &Path) -> Result<(), String> {
    // Use audio.ogg if it exists (cleaner headers), otherwise fall back to MKV
    let ogg_path = audio_path.with_file_name("audio.ogg");
    let input = if ogg_path.exists() { &ogg_path } else { audio_path };

    let output = Command::new("ffmpeg")
        .args(["-y", "-loglevel", "error"])
        .arg("-i").arg(input)
        .args([
            "-filter_complex",
            "showwavespic=s=320x80:colors=0xf43f5e",
            "-update", "1",
            "-frames:v", "1",
        ])
        .arg(output_path)
        .output()
        .map_err(|e| format!("Failed to run ffmpeg for audio waveform: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Audio waveform generation failed: {stderr}"));
    }
    Ok(())
}
