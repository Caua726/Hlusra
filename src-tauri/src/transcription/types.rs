use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptResult {
    pub language: String,
    pub segments: Vec<Segment>,
    pub full_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub words: Vec<Word>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WhisperModel {
    pub name: String,
    pub size_bytes: u64,
    pub downloaded: bool,
}

impl WhisperModel {
    /// Returns the filename used for this model on disk (e.g. "ggml-tiny.bin").
    pub fn filename(&self) -> String {
        format!("ggml-{}.bin", self.name)
    }

    /// Hugging Face URL for downloading the GGML model file.
    pub fn download_url(&self) -> String {
        format!(
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}.bin",
            self.name
        )
    }
}

/// Catalogue of all known Whisper model sizes with approximate file sizes.
pub fn all_models() -> Vec<WhisperModel> {
    vec![
        WhisperModel { name: "tiny".into(), size_bytes: 77_700_000, downloaded: false },
        WhisperModel { name: "base".into(), size_bytes: 147_000_000, downloaded: false },
        WhisperModel { name: "small".into(), size_bytes: 488_000_000, downloaded: false },
        WhisperModel { name: "medium".into(), size_bytes: 1_533_000_000, downloaded: false },
        WhisperModel { name: "large-v3".into(), size_bytes: 3_086_999_552, downloaded: false },
    ]
}
