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
        VideoFormat::MkvAv1 => "av1",
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

// resolve_output_path tests are in types.rs

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::types::VideoFormat;

    /// Helper: build the Command for video export and return its args as strings.
    /// Does NOT run ffmpeg — only inspects the argument list.
    /// `source_codec` simulates the result of `probe_video_codec`.
    fn build_video_cmd_args(format: VideoFormat, source_codec: &str) -> Vec<String> {
        let source = PathBuf::from("/fake/recording.mkv");
        let output = PathBuf::from("/fake/video.out");

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y")
            .arg("-loglevel")
            .arg("error")
            .arg("-i")
            .arg(&source);

        let target_codec = match format {
            VideoFormat::Mp4H264 | VideoFormat::MkvH264 => "h264",
            VideoFormat::Mp4H265 | VideoFormat::MkvH265 => "hevc",
            VideoFormat::MkvAv1 => "av1",
        };
        let can_copy = source_codec == target_codec;
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

        if format.is_mp4() {
            cmd.arg("-codec:a").arg("aac").arg("-b:a").arg("128k");
        } else {
            cmd.arg("-codec:a").arg("copy");
        }

        cmd.arg("-f").arg(format.container_name());

        if format.is_mp4() {
            cmd.arg("-movflags").arg("+faststart");
        }

        cmd.arg(&output);

        let debug = std::format!("{:?}", cmd);
        parse_command_args(&debug)
    }

    /// Parse args from the Debug representation of std::process::Command.
    fn parse_command_args(debug_str: &str) -> Vec<String> {
        let mut args = Vec::new();
        let mut chars = debug_str.chars().peekable();
        while let Some(&ch) = chars.peek() {
            if ch == '"' {
                chars.next();
                let mut arg = String::new();
                for c in chars.by_ref() {
                    if c == '"' {
                        break;
                    }
                    arg.push(c);
                }
                args.push(arg);
            } else {
                chars.next();
            }
        }
        args
    }

    #[test]
    fn test_video_mp4_h264_transcode_from_hevc() {
        // Source is hevc, target is h264 -> should transcode
        let args = build_video_cmd_args(VideoFormat::Mp4H264, "hevc");

        let codec_idx = args.iter().position(|a| a == "-codec:v").unwrap();
        assert_eq!(args[codec_idx + 1], "libx264", "should transcode to libx264");
        assert!(args.contains(&"-preset".to_string()));
        assert!(args.contains(&"-crf".to_string()));

        // MP4 -> audio should be AAC
        let audio_idx = args.iter().position(|a| a == "-codec:a").unwrap();
        assert_eq!(args[audio_idx + 1], "aac");

        // MP4 -> should have -movflags +faststart
        let mov_idx = args.iter().position(|a| a == "-movflags").unwrap();
        assert_eq!(args[mov_idx + 1], "+faststart");

        // Container
        let f_idx = args.iter().position(|a| a == "-f").unwrap();
        assert_eq!(args[f_idx + 1], "mp4");
    }

    #[test]
    fn test_video_mp4_h265_stream_copy_from_hevc() {
        // Source is hevc, target is h265 -> should stream-copy
        let args = build_video_cmd_args(VideoFormat::Mp4H265, "hevc");

        let codec_idx = args.iter().position(|a| a == "-codec:v").unwrap();
        assert_eq!(args[codec_idx + 1], "copy", "matching codec should stream-copy");
        assert!(!args.contains(&"-preset".to_string()), "no preset for stream-copy");
        assert!(!args.contains(&"-crf".to_string()), "no crf for stream-copy");
    }

    #[test]
    fn test_video_mkv_h264_stream_copy_from_h264() {
        // Source is h264, target is MkvH264 -> should stream-copy
        let args = build_video_cmd_args(VideoFormat::MkvH264, "h264");

        let codec_idx = args.iter().position(|a| a == "-codec:v").unwrap();
        assert_eq!(args[codec_idx + 1], "copy");

        // MKV -> audio should be copied
        let audio_idx = args.iter().position(|a| a == "-codec:a").unwrap();
        assert_eq!(args[audio_idx + 1], "copy");

        // Container
        let f_idx = args.iter().position(|a| a == "-f").unwrap();
        assert_eq!(args[f_idx + 1], "matroska");

        // MKV should NOT have -movflags
        assert!(!args.contains(&"-movflags".to_string()));
    }

    #[test]
    fn test_video_mkv_av1_transcode_from_hevc() {
        // Source is hevc, target is AV1 -> should transcode
        let args = build_video_cmd_args(VideoFormat::MkvAv1, "hevc");

        let codec_idx = args.iter().position(|a| a == "-codec:v").unwrap();
        assert_eq!(args[codec_idx + 1], "libsvtav1");

        // MKV -> audio copied
        let audio_idx = args.iter().position(|a| a == "-codec:a").unwrap();
        assert_eq!(args[audio_idx + 1], "copy");

        let f_idx = args.iter().position(|a| a == "-f").unwrap();
        assert_eq!(args[f_idx + 1], "matroska");
    }

    #[test]
    fn test_video_mkv_h265_transcode_from_h264() {
        // Source is h264, target is MkvH265 -> should transcode to libx265
        let args = build_video_cmd_args(VideoFormat::MkvH265, "h264");

        let codec_idx = args.iter().position(|a| a == "-codec:v").unwrap();
        assert_eq!(args[codec_idx + 1], "libx265");
        assert!(args.contains(&"-preset".to_string()));
    }

    #[test]
    fn test_video_common_args_present() {
        // Verify all commands have -y, -loglevel error, -i
        let args = build_video_cmd_args(VideoFormat::Mp4H264, "hevc");
        assert_eq!(args[0], "ffmpeg");
        assert_eq!(args[1], "-y");
        assert_eq!(args[2], "-loglevel");
        assert_eq!(args[3], "error");
        assert_eq!(args[4], "-i");
    }
}
