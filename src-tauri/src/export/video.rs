use std::path::{Path, PathBuf};
use std::process::Command;

use super::types::{resolve_output_path, SaveMode, VideoFormat};
use super::ExportError;

/// Probe the video codec of a source file using ffprobe.
/// Returns the codec name (e.g. "hevc", "h264", "av1").
fn probe_video_codec(source: &Path) -> Result<String, ExportError> {
    let output = Command::new("ffprobe")
        .args(["-v", "error", "-select_streams", "v:0",
               "-show_entries", "stream=codec_name", "-of", "csv=p=0"])
        .arg(source)
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(ExportError::FfmpegFailed(format!("ffprobe failed: {}", stderr)));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Export video from a meeting's recording.mkv to the specified format.
///
/// Transcodes from the source MKV (H.265) to the target codec and container.
/// When the target codec matches the source, stream-copies the video to avoid
/// re-encoding. For MP4 output, audio is transcoded to AAC (Opus is not valid
/// in MP4). For MKV output, audio is copied as-is.
///
/// Uses FFmpeg CLI for media processing (MVP approach).
pub fn export_video(
    meeting_dir: &Path,
    format: VideoFormat,
    save_mode: &SaveMode,
) -> Result<PathBuf, ExportError> {
    let source = meeting_dir.join("recording.mkv");
    if !source.exists() {
        return Err(ExportError::SourceNotFound(source));
    }

    let filename = format!("video.{}", format.extension());
    let output_path = resolve_output_path(meeting_dir, &filename, save_mode);

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-y") // overwrite
        .arg("-loglevel")
        .arg("error")
        .arg("-i")
        .arg(&source);

    // Probe the actual source codec to decide stream-copy vs transcode.
    let source_codec = probe_video_codec(&source).unwrap_or_else(|_| "unknown".to_string());
    let target_codec = match format {
        VideoFormat::Mp4H264 | VideoFormat::MkvH264 => "h264",
        VideoFormat::Mp4H265 | VideoFormat::MkvH265 => "hevc",
    };
    // Stream-copy only if source matches target codec exactly
    let can_copy = source_codec == target_codec
        || (target_codec == "hevc" && source_codec == "hevc")
        || (target_codec == "h264" && source_codec == "h264");
    if can_copy {
        cmd.arg("-codec:v").arg("copy");
    } else {
        cmd.arg("-codec:v")
            .arg(format.codec_name())
            .arg("-preset")
            .arg("medium")
            .arg("-crf")
            .arg("20");
    }

    // Audio handling: MP4 cannot hold Opus, so transcode to AAC.
    // MKV supports Opus natively, so we can stream-copy.
    if format.is_mp4() {
        cmd.arg("-codec:a").arg("aac").arg("-b:a").arg("128k");
    } else {
        cmd.arg("-codec:a").arg("copy");
    }

    // Output format
    cmd.arg("-f").arg(format.container_name());

    // For MP4, move the moov atom to the beginning for web streaming
    if format.is_mp4() {
        cmd.arg("-movflags").arg("+faststart");
    }

    cmd.arg(&output_path);

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(ExportError::FfmpegFailed(stderr));
    }

    Ok(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_output_save() {
        let dir = PathBuf::from("/meetings/abc123");
        let path = resolve_output_path(&dir, "video.mp4", &SaveMode::Save);
        assert_eq!(path, PathBuf::from("/meetings/abc123/video.mp4"));
    }

    #[test]
    fn test_resolve_output_save_as() {
        let dir = PathBuf::from("/meetings/abc123");
        let save_as = SaveMode::SaveAs {
            path: PathBuf::from("/home/user/meeting.mp4"),
        };
        let path = resolve_output_path(&dir, "video.mp4", &save_as);
        assert_eq!(path, PathBuf::from("/home/user/meeting.mp4"));
    }
}
