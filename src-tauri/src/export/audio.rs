use std::path::{Path, PathBuf};
use std::process::Command;

use super::types::{resolve_output_path, AudioFormat, SaveMode};
use super::ExportError;

/// Probe the source file with ffprobe and return the number of audio streams.
fn count_audio_streams(source: &Path) -> Result<usize, ExportError> {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("a")
        .arg("-show_entries")
        .arg("stream=index")
        .arg("-of")
        .arg("csv=p=0")
        .arg(source)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(ExportError::FfmpegFailed(format!(
            "ffprobe failed: {}",
            stderr
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let count = stdout.lines().filter(|l| !l.trim().is_empty()).count();
    Ok(count)
}

/// Build the amerge + pan filter_complex string for N audio streams.
///
/// For N streams the filter looks like:
///   `[0:a:0][0:a:1]...[0:a:N-1]amerge=inputs=N,pan=stereo|c0<c0+c2+...+cE|c1<c1+c3+...+cO[aout]`
///
/// The pan formula distributes even-numbered channels to the left output and
/// odd-numbered channels to the right output so that each source stream
/// (originally mono) is placed evenly in the stereo field.
fn build_mixdown_filter(stream_count: usize) -> String {
    // Build input pad labels: [0:a:0][0:a:1]...
    let input_pads: String = (0..stream_count)
        .map(|i| format!("[0:a:{}]", i))
        .collect();

    // Total channels after amerge = stream_count (each stream assumed mono)
    let total_channels = stream_count;

    // Build pan formula: even channels -> c0 (left), odd channels -> c1 (right)
    let left_channels: Vec<String> = (0..total_channels)
        .filter(|c| c % 2 == 0)
        .map(|c| format!("c{}", c))
        .collect();
    let right_channels: Vec<String> = (0..total_channels)
        .filter(|c| c % 2 == 1)
        .map(|c| format!("c{}", c))
        .collect();

    // If all channels are even (single stream), duplicate to both sides
    let left = left_channels.join("+");
    let right = if right_channels.is_empty() {
        left.clone()
    } else {
        right_channels.join("+")
    };

    format!(
        "{}amerge=inputs={},pan=stereo|c0<{}|c1<{}[aout]",
        input_pads, stream_count, left, right
    )
}

/// Export audio from a meeting's recording.mkv to the specified format.
///
/// For formats that require mixdown, all audio tracks are mixed into a single
/// stereo stream. If the source has only one audio track, `-ac 2` is applied
/// to guarantee stereo output from a potentially mono source.
///
/// Uses FFmpeg CLI for media processing (MVP approach).
pub fn export_audio(
    meeting_dir: &Path,
    format: AudioFormat,
    save_mode: &SaveMode,
) -> Result<PathBuf, ExportError> {
    let source = meeting_dir.join("recording.mkv");
    if !source.exists() {
        return Err(ExportError::SourceNotFound(source));
    }

    let filename = format!("audio.{}", format.extension());
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

    if format.requires_mixdown() {
        let stream_count = count_audio_streams(&source)?;

        if stream_count == 0 {
            return Err(ExportError::FfmpegFailed(
                "Source file contains no audio streams".to_string(),
            ));
        }

        if stream_count >= 2 {
            // Merge multiple audio tracks into a single stereo stream
            let filter = build_mixdown_filter(stream_count);
            cmd.arg("-filter_complex")
                .arg(&filter)
                .arg("-map")
                .arg("[aout]");
        } else {
            // Single audio stream — force stereo output for mono sources
            cmd.arg("-ac").arg("2");
        }
    }

    // No video output
    cmd.arg("-vn");

    match format {
        AudioFormat::Mp3 => {
            cmd.arg("-codec:a").arg("libmp3lame").arg("-q:a").arg("2");
        }
        AudioFormat::Wav => {
            cmd.arg("-codec:a").arg("pcm_s16le");
        }
        AudioFormat::Opus => {
            cmd.arg("-codec:a").arg("libopus");
        }
        AudioFormat::Ogg => {
            cmd.arg("-codec:a").arg("libvorbis").arg("-q:a").arg("4");
        }
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
    fn test_build_mixdown_filter_2_streams() {
        let filter = build_mixdown_filter(2);
        assert_eq!(
            filter,
            "[0:a:0][0:a:1]amerge=inputs=2,pan=stereo|c0<c0|c1<c1[aout]"
        );
    }

    #[test]
    fn test_build_mixdown_filter_3_streams() {
        let filter = build_mixdown_filter(3);
        assert_eq!(
            filter,
            "[0:a:0][0:a:1][0:a:2]amerge=inputs=3,pan=stereo|c0<c0+c2|c1<c1[aout]"
        );
    }

    #[test]
    fn test_build_mixdown_filter_1_stream() {
        // Single stream: both left and right get c0
        let filter = build_mixdown_filter(1);
        assert_eq!(
            filter,
            "[0:a:0]amerge=inputs=1,pan=stereo|c0<c0|c1<c0[aout]"
        );
    }

    // --- S39: FFmpeg argument-construction tests ---

    /// Helper: build the Command for audio export and return its args as strings.
    /// Does NOT run ffmpeg — only inspects the argument list.
    fn build_audio_cmd_args(format: AudioFormat, stream_count: usize) -> Vec<String> {
        let source = PathBuf::from("/fake/recording.mkv");
        let output = PathBuf::from("/fake/audio.out");

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y")
            .arg("-loglevel")
            .arg("error")
            .arg("-i")
            .arg(&source);

        if format.requires_mixdown() {
            if stream_count >= 2 {
                let filter = build_mixdown_filter(stream_count);
                cmd.arg("-filter_complex")
                    .arg(&filter)
                    .arg("-map")
                    .arg("[aout]");
            } else {
                cmd.arg("-ac").arg("2");
            }
        }

        cmd.arg("-vn");

        match format {
            AudioFormat::Mp3 => {
                cmd.arg("-codec:a").arg("libmp3lame").arg("-q:a").arg("2");
            }
            AudioFormat::Wav => {
                cmd.arg("-codec:a").arg("pcm_s16le");
            }
            AudioFormat::Opus => {
                cmd.arg("-codec:a").arg("libopus");
            }
            AudioFormat::Ogg => {
                cmd.arg("-codec:a").arg("libvorbis").arg("-q:a").arg("4");
            }
        }

        cmd.arg(&output);

        // Extract args from the Command via Debug formatting
        let debug = format!("{:?}", cmd);
        parse_command_args(&debug)
    }

    /// Parse args from the Debug representation of std::process::Command.
    /// Format: `"ffmpeg" "-y" "-loglevel" "error" ...`
    fn parse_command_args(debug_str: &str) -> Vec<String> {
        let mut args = Vec::new();
        let mut chars = debug_str.chars().peekable();
        while let Some(&ch) = chars.peek() {
            if ch == '"' {
                chars.next(); // consume opening quote
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
    fn test_audio_export_mp3_args_multi_stream() {
        let args = build_audio_cmd_args(AudioFormat::Mp3, 2);

        // Should contain the amerge filter for 2 streams
        let filter_idx = args.iter().position(|a| a == "-filter_complex").unwrap();
        let filter_val = &args[filter_idx + 1];
        assert!(
            filter_val.contains("[0:a:0][0:a:1]"),
            "filter should reference both input pads"
        );
        assert!(
            filter_val.contains("amerge=inputs=2"),
            "filter should amerge 2 inputs"
        );

        // Should map [aout]
        let map_idx = args.iter().position(|a| a == "-map").unwrap();
        assert_eq!(args[map_idx + 1], "[aout]");

        // Should use libmp3lame codec
        let codec_idx = args.iter().position(|a| a == "-codec:a").unwrap();
        assert_eq!(args[codec_idx + 1], "libmp3lame");

        // Should have -vn (no video)
        assert!(args.contains(&"-vn".to_string()));
    }

    #[test]
    fn test_audio_export_single_stream_no_filter() {
        let args = build_audio_cmd_args(AudioFormat::Mp3, 1);

        // For 1 stream, no filter_complex should be applied
        assert!(
            !args.contains(&"-filter_complex".to_string()),
            "single stream should not use -filter_complex"
        );

        // Instead, -ac 2 should be present
        let ac_idx = args.iter().position(|a| a == "-ac").unwrap();
        assert_eq!(args[ac_idx + 1], "2");
    }

    #[test]
    fn test_audio_export_wav_args() {
        let args = build_audio_cmd_args(AudioFormat::Wav, 1);

        let codec_idx = args.iter().position(|a| a == "-codec:a").unwrap();
        assert_eq!(args[codec_idx + 1], "pcm_s16le");
    }

    #[test]
    fn test_audio_export_opus_args() {
        let args = build_audio_cmd_args(AudioFormat::Opus, 3);

        let codec_idx = args.iter().position(|a| a == "-codec:a").unwrap();
        assert_eq!(args[codec_idx + 1], "libopus");

        // 3 streams should have filter_complex with amerge=inputs=3
        let filter_idx = args.iter().position(|a| a == "-filter_complex").unwrap();
        let filter_val = &args[filter_idx + 1];
        assert!(filter_val.contains("amerge=inputs=3"));
    }

    #[test]
    fn test_audio_export_ogg_args() {
        let args = build_audio_cmd_args(AudioFormat::Ogg, 1);

        let codec_idx = args.iter().position(|a| a == "-codec:a").unwrap();
        assert_eq!(args[codec_idx + 1], "libvorbis");

        // Quality flag
        let q_idx = args.iter().position(|a| a == "-q:a").unwrap();
        assert_eq!(args[q_idx + 1], "4");
    }

    #[test]
    fn test_build_mixdown_filter_4_streams() {
        let filter = build_mixdown_filter(4);
        // 4 streams: pads [0:a:0][0:a:1][0:a:2][0:a:3]
        assert!(filter.starts_with("[0:a:0][0:a:1][0:a:2][0:a:3]amerge=inputs=4"));
        // Even channels (c0, c2) -> left; odd channels (c1, c3) -> right
        assert!(filter.contains("c0<c0+c2"));
        assert!(filter.contains("c1<c1+c3"));
    }
}
