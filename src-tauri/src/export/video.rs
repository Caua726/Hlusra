use std::path::{Path, PathBuf};
use std::process::Command;

use super::types::{SaveMode, VideoFormat};
use super::ExportError;

/// Export video from a meeting's recording.mkv to the specified format.
///
/// Transcodes from the source MKV (H.265) to the target codec and container.
/// When the target codec matches the source, stream-copies the video to avoid
/// re-encoding. Audio is copied as-is.
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
        .arg("-i")
        .arg(&source);

    // Determine if we can stream-copy video or need to transcode.
    // Source is H.265/MKV. If target is also H.265, we can copy the stream.
    match format {
        VideoFormat::Mp4H265 | VideoFormat::MkvH265 => {
            // Same codec, stream copy
            cmd.arg("-codec:v").arg("copy");
        }
        VideoFormat::Mp4H264 | VideoFormat::MkvH264 => {
            // Need to transcode from H.265 to H.264
            cmd.arg("-codec:v").arg("libx264").arg("-preset").arg("medium");
        }
    }

    // Copy audio streams
    cmd.arg("-codec:a").arg("copy");

    // Output format
    cmd.arg("-f").arg(format.container_name());

    cmd.arg(&output_path);

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(ExportError::FfmpegFailed(stderr));
    }

    Ok(output_path)
}

/// Resolve the final output path based on the save mode.
fn resolve_output_path(meeting_dir: &Path, filename: &str, save_mode: &SaveMode) -> PathBuf {
    match save_mode {
        SaveMode::Save => meeting_dir.join(filename),
        SaveMode::SaveAs { path } => path.clone(),
    }
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
