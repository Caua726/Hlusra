pub mod types;
pub mod provider;
pub mod local;
pub mod api;
pub mod orchestrator;
pub mod models;
pub mod commands;

pub use types::{TranscriptResult, Segment, Word, WhisperModel};
pub use provider::TranscriptionProvider;
