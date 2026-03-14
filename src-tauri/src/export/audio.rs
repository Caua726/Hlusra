use std::path::{Path, PathBuf};
use std::process::Command;

use super::types::{AudioFormat, SaveMode};
use super::ExportError;

/// Export audio from a meeting's recording.mkv to the specified format.
///
/// For formats that require mixdown (MP3, WAV, OGG), all audio tracks are
/// mixed into a single mono/stereo stream. Opus can preserve tracks in
/// supported containers.
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
        // Mix all audio tracks into a single stream
        cmd.arg("-filter_complex")
            .arg("amix=inputs=1:duration=longest")
            .arg("-ac")
            .arg("1");
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
