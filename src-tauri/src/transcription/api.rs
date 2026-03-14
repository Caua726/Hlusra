use std::path::Path;
use std::time::Duration;

use reqwest::blocking::multipart;
use serde::Deserialize;

use crate::transcription::provider::TranscriptionProvider;
use crate::transcription::types::{Segment, TranscriptResult, Word};

/// Transcription provider that calls an OpenAI Whisper-compatible HTTP endpoint.
pub struct ApiProvider {
    /// Base URL of the API (e.g. "https://api.openai.com" or "http://localhost:8080").
    base_url: String,
    /// Bearer token sent in the Authorization header. May be empty for local servers.
    api_key: String,
    /// Model identifier sent in the form field (e.g. "whisper-1").
    model: String,
    /// Reusable HTTP client with a long timeout for transcription requests.
    client: reqwest::blocking::Client,
}

/// Matches the OpenAI verbose JSON response for `/v1/audio/transcriptions`.
#[derive(Debug, Deserialize)]
struct ApiResponse {
    #[serde(default)]
    language: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    segments: Vec<ApiSegment>,
}

#[derive(Debug, Deserialize)]
struct ApiSegment {
    start: f64,
    end: f64,
    text: String,
    #[serde(default)]
    words: Vec<ApiWord>,
}

#[derive(Debug, Deserialize)]
struct ApiWord {
    start: f64,
    end: f64,
    word: String,
    #[serde(default)]
    probability: f32,
}

impl ApiProvider {
    pub fn new(base_url: String, api_key: String, model: String) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
            .expect("Failed to build HTTP client");
        Self {
            base_url,
            api_key,
            model,
            client,
        }
    }
}

impl TranscriptionProvider for ApiProvider {
    fn transcribe(&self, audio_path: &Path) -> Result<TranscriptResult, String> {
        let url = format!(
            "{}/v1/audio/transcriptions",
            self.base_url.trim_end_matches('/')
        );

        let file_part = multipart::Part::file(audio_path)
            .map_err(|e| format!("Failed to read audio file for upload: {e}"))?
            .file_name("audio.wav")
            .mime_str("audio/wav")
            .unwrap();

        let form = multipart::Form::new()
            .part("file", file_part)
            .text("model", self.model.clone())
            .text("response_format", "verbose_json")
            .text("timestamp_granularities[]", "word");

        let mut request = self.client.post(&url).multipart(form);

        if !self.api_key.is_empty() {
            request = request.bearer_auth(&self.api_key);
        }

        let response = request
            .send()
            .map_err(|e| format!("API request failed: {e}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .unwrap_or_else(|_| "unable to read body".into());
            return Err(format!("API returned {status}: {body}"));
        }

        let api_resp: ApiResponse = response
            .json()
            .map_err(|e| format!("Failed to parse API response: {e}"))?;

        // Convert the API response into our internal representation.
        let segments: Vec<Segment> = api_resp
            .segments
            .into_iter()
            .map(|s| Segment {
                start: s.start,
                end: s.end,
                text: s.text.trim().to_string(),
                words: s
                    .words
                    .into_iter()
                    .map(|w| Word {
                        start: w.start,
                        end: w.end,
                        text: w.word,
                        confidence: w.probability,
                    })
                    .collect(),
            })
            .collect();

        let full_text = if api_resp.text.is_empty() {
            segments
                .iter()
                .map(|s| s.text.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            api_resp.text
        };

        Ok(TranscriptResult {
            language: api_resp.language,
            segments,
            full_text,
        })
    }
}
