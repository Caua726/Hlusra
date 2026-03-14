use std::path::Path;
use crate::transcription::types::TranscriptResult;

/// Common contract for all transcription backends.
pub trait TranscriptionProvider: Send + Sync {
    fn transcribe(&self, audio_path: &Path) -> Result<TranscriptResult, String>;
}
