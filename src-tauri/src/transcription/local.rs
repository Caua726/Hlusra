use std::path::Path;

use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::transcription::models::models_dir;
use crate::transcription::provider::TranscriptionProvider;
use crate::transcription::types::{Segment, TranscriptResult, WhisperModel, Word};

/// Transcription provider that runs whisper.cpp in-process via whisper-rs.
pub struct LocalProvider {
    model: WhisperModel,
}

impl LocalProvider {
    pub fn new(model: WhisperModel) -> Self {
        Self { model }
    }

    /// Reads a WAV file (expected 16 kHz mono PCM s16le) and returns f32 samples.
    fn load_wav_samples(audio_path: &Path) -> Result<Vec<f32>, String> {
        let reader = hound::WavReader::open(audio_path)
            .map_err(|e| format!("Failed to open WAV file: {e}"))?;
        let spec = reader.spec();
        if spec.channels != 1 {
            return Err(format!("Expected mono audio, got {} channels", spec.channels));
        }
        if spec.sample_rate != 16000 {
            return Err(format!(
                "Expected 16000 Hz sample rate, got {}",
                spec.sample_rate
            ));
        }

        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Int => {
                let max = (1i64 << (spec.bits_per_sample - 1)) as f32;
                reader
                    .into_samples::<i32>()
                    .filter_map(|s| s.ok())
                    .map(|s| s as f32 / max)
                    .collect()
            }
            hound::SampleFormat::Float => reader
                .into_samples::<f32>()
                .filter_map(|s| s.ok())
                .collect(),
        };

        Ok(samples)
    }
}

impl TranscriptionProvider for LocalProvider {
    fn transcribe(&self, audio_path: &Path) -> Result<TranscriptResult, String> {
        let model_path = models_dir()
            .map_err(|e| format!("Cannot determine models directory: {e}"))?
            .join(self.model.filename());

        if !model_path.exists() {
            return Err(format!(
                "Model file not found: {}. Download it first.",
                model_path.display()
            ));
        }

        // Load audio samples from the WAV file.
        let samples = Self::load_wav_samples(audio_path)?;

        // Create whisper context from the model file.
        let ctx = WhisperContext::new_with_params(
            model_path.to_str().ok_or("Invalid model path encoding")?,
            WhisperContextParameters::default(),
        )
        .map_err(|e| format!("Failed to load whisper model: {e}"))?;

        let mut state = ctx
            .create_state()
            .map_err(|e| format!("Failed to create whisper state: {e}"))?;

        // Configure inference parameters.
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_token_timestamps(true);
        params.set_language(None); // auto-detect

        // Run inference.
        state
            .full(params, &samples)
            .map_err(|e| format!("Whisper inference failed: {e}"))?;

        // Extract results.
        let num_segments = state.full_n_segments().map_err(|e| format!("Failed to get segment count: {e}"))?;
        let mut segments = Vec::with_capacity(num_segments as usize);
        let mut full_text = String::new();
        let mut detected_language = String::from("auto");

        // Try to get the detected language from the first segment.
        if num_segments > 0 {
            if let Ok(lang_id) = state.full_lang_id_from_state() {
                if let Some(lang) = whisper_rs::get_lang_str(lang_id) {
                    detected_language = lang.to_string();
                }
            }
        }

        for i in 0..num_segments {
            let segment_text = state
                .full_get_segment_text(i)
                .map_err(|e| format!("Failed to get segment {i} text: {e}"))?;

            let t0 = state
                .full_get_segment_t0(i)
                .map_err(|e| format!("Failed to get segment {i} start time: {e}"))?;
            let t1 = state
                .full_get_segment_t1(i)
                .map_err(|e| format!("Failed to get segment {i} end time: {e}"))?;

            // whisper-rs times are in centiseconds (hundredths of a second).
            let seg_start = t0 as f64 / 100.0;
            let seg_end = t1 as f64 / 100.0;

            // Extract word-level timestamps.
            let num_tokens = state
                .full_n_tokens(i)
                .map_err(|e| format!("Failed to get token count for segment {i}: {e}"))?;

            let mut words = Vec::new();
            for j in 0..num_tokens {
                let token_data = state
                    .full_get_token_data(i, j)
                    .map_err(|e| format!("Failed to get token data ({i},{j}): {e}"))?;

                let token_text = state
                    .full_get_token_text(i, j)
                    .map_err(|e| format!("Failed to get token text ({i},{j}): {e}"))?;

                // Skip special tokens (they start with '[' or are empty/whitespace-only).
                let trimmed = token_text.trim();
                if trimmed.is_empty()
                    || trimmed.starts_with('[')
                    || trimmed.starts_with("<|")
                {
                    continue;
                }

                words.push(Word {
                    start: token_data.t0 as f64 / 100.0,
                    end: token_data.t1 as f64 / 100.0,
                    text: token_text,
                    confidence: token_data.p,
                });
            }

            if !full_text.is_empty() && !segment_text.starts_with(' ') {
                full_text.push(' ');
            }
            full_text.push_str(segment_text.trim());

            segments.push(Segment {
                start: seg_start,
                end: seg_end,
                text: segment_text.trim().to_string(),
                words,
            });
        }

        Ok(TranscriptResult {
            language: detected_language,
            segments,
            full_text,
        })
    }
}
