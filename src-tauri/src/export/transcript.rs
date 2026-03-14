use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use super::types::{SaveMode, TranscriptFormat};
use super::ExportError;

/// A segment from the transcript.json file, used for SRT and PDF generation.
#[derive(Debug, Clone, Deserialize)]
pub struct TranscriptSegment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

/// Export a meeting's transcript to the specified format.
///
/// - TXT: copies the existing transcript.txt
/// - JSON: copies the existing transcript.json
/// - SRT: generates subtitle file from transcript.json segments
/// - PDF: generates a formatted PDF from transcript.json segments via genpdf
pub fn export_transcript(
    meeting_dir: &Path,
    format: TranscriptFormat,
    save_mode: &SaveMode,
) -> Result<PathBuf, ExportError> {
    let filename = format!("transcript.{}", format.extension());
    let output_path = resolve_output_path(meeting_dir, &filename, save_mode);

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    match format {
        TranscriptFormat::Txt => export_txt(meeting_dir, &output_path),
        TranscriptFormat::Json => export_json(meeting_dir, &output_path),
        TranscriptFormat::Srt => export_srt(meeting_dir, &output_path),
        TranscriptFormat::Pdf => export_pdf(meeting_dir, &output_path),
    }
}

/// Copy existing transcript.txt to the output path.
fn export_txt(meeting_dir: &Path, output_path: &Path) -> Result<PathBuf, ExportError> {
    let source = meeting_dir.join("transcript.txt");
    if !source.exists() {
        return Err(ExportError::SourceNotFound(source));
    }
    fs::copy(&source, output_path)?;
    Ok(output_path.to_path_buf())
}

/// Copy existing transcript.json to the output path.
fn export_json(meeting_dir: &Path, output_path: &Path) -> Result<PathBuf, ExportError> {
    let source = meeting_dir.join("transcript.json");
    if !source.exists() {
        return Err(ExportError::SourceNotFound(source));
    }
    fs::copy(&source, output_path)?;
    Ok(output_path.to_path_buf())
}

/// Generate an SRT subtitle file from transcript.json segments.
///
/// SRT format:
/// ```text
/// 1
/// 00:00:01,500 --> 00:00:04,200
/// Hello, welcome to the meeting.
///
/// 2
/// 00:00:04,500 --> 00:00:08,000
/// Let's discuss the agenda.
/// ```
fn export_srt(meeting_dir: &Path, output_path: &Path) -> Result<PathBuf, ExportError> {
    let segments = load_segments(meeting_dir)?;
    let mut srt = String::new();

    for (i, segment) in segments.iter().enumerate() {
        let index = i + 1;
        let start = format_srt_timestamp(segment.start);
        let end = format_srt_timestamp(segment.end);
        let text = segment.text.trim();

        srt.push_str(&format!("{}\n{} --> {}\n{}\n\n", index, start, end, text));
    }

    fs::write(output_path, srt)?;
    Ok(output_path.to_path_buf())
}

/// Generate a PDF document from transcript.json segments via genpdf.
fn export_pdf(meeting_dir: &Path, output_path: &Path) -> Result<PathBuf, ExportError> {
    let segments = load_segments(meeting_dir)?;

    // Use genpdf's built-in default font
    let font_family = genpdf::fonts::from_files("", "LiberationSans", None)
        .unwrap_or_else(|_| {
            // Fallback: try system font paths common on Linux
            genpdf::fonts::from_files("/usr/share/fonts/TTF", "LiberationSans", None)
                .or_else(|_| {
                    genpdf::fonts::from_files(
                        "/usr/share/fonts/truetype/liberation",
                        "LiberationSans",
                        None,
                    )
                })
                .or_else(|_| {
                    genpdf::fonts::from_files(
                        "/usr/share/fonts/liberation-sans",
                        "LiberationSans",
                        None,
                    )
                })
                .unwrap_or_else(|_| {
                    // Last resort: try DejaVu which is very common
                    genpdf::fonts::from_files("/usr/share/fonts/TTF", "DejaVuSans", None)
                        .or_else(|_| {
                            genpdf::fonts::from_files(
                                "/usr/share/fonts/truetype/dejavu",
                                "DejaVuSans",
                                None,
                            )
                        })
                        .expect("No suitable font found. Install liberation-fonts or dejavu-fonts.")
                })
        });

    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Meeting Transcript");

    // Set document margins
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // Title
    doc.push(genpdf::elements::Paragraph::new("Meeting Transcript").styled(
        genpdf::style::Style::new().bold().with_font_size(18),
    ));
    doc.push(genpdf::elements::Break::new(1.0));

    // Transcript segments
    for segment in &segments {
        let timestamp = format!(
            "[{} - {}]",
            format_readable_timestamp(segment.start),
            format_readable_timestamp(segment.end)
        );

        doc.push(
            genpdf::elements::Paragraph::new(timestamp)
                .styled(genpdf::style::Style::new().bold().with_font_size(9)),
        );
        doc.push(genpdf::elements::Paragraph::new(segment.text.trim()));
        doc.push(genpdf::elements::Break::new(0.5));
    }

    doc.render_to_file(output_path)
        .map_err(|e| ExportError::PdfGeneration(e.to_string()))?;

    Ok(output_path.to_path_buf())
}

/// Load transcript segments from transcript.json.
fn load_segments(meeting_dir: &Path) -> Result<Vec<TranscriptSegment>, ExportError> {
    let json_path = meeting_dir.join("transcript.json");
    if !json_path.exists() {
        return Err(ExportError::SourceNotFound(json_path));
    }

    let content = fs::read_to_string(&json_path)?;
    let segments: Vec<TranscriptSegment> =
        serde_json::from_str(&content).map_err(|e| ExportError::InvalidTranscript(e.to_string()))?;
    Ok(segments)
}

/// Format seconds as SRT timestamp: HH:MM:SS,mmm
fn format_srt_timestamp(seconds: f64) -> String {
    let total_ms = (seconds * 1000.0).round() as u64;
    let ms = total_ms % 1000;
    let total_secs = total_ms / 1000;
    let secs = total_secs % 60;
    let total_mins = total_secs / 60;
    let mins = total_mins % 60;
    let hours = total_mins / 60;

    format!("{:02}:{:02}:{:02},{:03}", hours, mins, secs, ms)
}

/// Format seconds as readable timestamp: HH:MM:SS
fn format_readable_timestamp(seconds: f64) -> String {
    let total_secs = seconds.round() as u64;
    let secs = total_secs % 60;
    let total_mins = total_secs / 60;
    let mins = total_mins % 60;
    let hours = total_mins / 60;

    format!("{:02}:{:02}:{:02}", hours, mins, secs)
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
    fn test_format_srt_timestamp() {
        assert_eq!(format_srt_timestamp(0.0), "00:00:00,000");
        assert_eq!(format_srt_timestamp(1.5), "00:00:01,500");
        assert_eq!(format_srt_timestamp(61.234), "00:01:01,234");
        assert_eq!(format_srt_timestamp(3661.999), "01:01:02,000"); // rounding
        assert_eq!(format_srt_timestamp(3600.0), "01:00:00,000");
    }

    #[test]
    fn test_format_readable_timestamp() {
        assert_eq!(format_readable_timestamp(0.0), "00:00:00");
        assert_eq!(format_readable_timestamp(90.0), "00:01:30");
        assert_eq!(format_readable_timestamp(3661.0), "01:01:01");
    }

    #[test]
    fn test_resolve_output_save() {
        let dir = PathBuf::from("/meetings/abc");
        let path = resolve_output_path(&dir, "transcript.srt", &SaveMode::Save);
        assert_eq!(path, PathBuf::from("/meetings/abc/transcript.srt"));
    }

    #[test]
    fn test_resolve_output_save_as() {
        let dir = PathBuf::from("/meetings/abc");
        let save_as = SaveMode::SaveAs {
            path: PathBuf::from("/home/user/transcript.pdf"),
        };
        let path = resolve_output_path(&dir, "transcript.pdf", &save_as);
        assert_eq!(path, PathBuf::from("/home/user/transcript.pdf"));
    }
}
