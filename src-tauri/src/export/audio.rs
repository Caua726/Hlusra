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

/// Export audio from a meeting's recording.mkv to the specified format.
///
/// For formats that require mixdown (MP3, WAV, OGG), all audio tracks are
/// mixed into a single stereo stream. If the source has only one audio track,
/// no mixing filter is applied. Opus can preserve tracks in supported
/// containers.
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
        .arg("-i")
        .arg(&source);

    if format.requires_mixdown() {
        let stream_count = count_audio_streams(&source)?;

        if stream_count >= 2 {
            // Merge multiple audio tracks into a single stereo stream
            cmd.arg("-filter_complex")
                .arg("[0:a]amerge=inputs=2,pan=stereo|c0<c0+c1|c1<c0+c1[aout]")
                .arg("-map")
                .arg("[aout]");
        }
        // For a single audio track, no filter is needed — FFmpeg selects it automatically.
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
    fn test_resolve_output_save() {
        let dir = PathBuf::from("/meetings/abc123");
        let path = resolve_output_path(&dir, "audio.mp3", &SaveMode::Save);
        assert_eq!(path, PathBuf::from("/meetings/abc123/audio.mp3"));
    }

    #[test]
    fn test_resolve_output_save_as() {
        let dir = PathBuf::from("/meetings/abc123");
        let save_as = SaveMode::SaveAs {
            path: PathBuf::from("/home/user/export.wav"),
        };
        let path = resolve_output_path(&dir, "audio.wav", &save_as);
        assert_eq!(path, PathBuf::from("/home/user/export.wav"));
    }
}
