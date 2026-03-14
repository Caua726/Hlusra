use std::path::Path;
use std::process::Command;

pub fn generate_thumbnail(video_path: &Path, output_path: &Path) -> Result<(), String> {
    let output = Command::new("ffmpeg")
        .args(["-y", "-loglevel", "error"])
        .arg("-i").arg(video_path)
        .args(["-ss", "10", "-vframes", "1", "-vf", "scale=320:-1"])
        .arg(output_path)
        .output()
        .map_err(|e| format!("Failed to run ffmpeg for thumbnail: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Thumbnail generation failed: {stderr}"));
    }
    Ok(())
}
