use serde::{Deserialize, Serialize};

// Re-export ChatStatus from library — do NOT redefine it here.
// Use: `use crate::library::types::ChatStatus;`

/// Error response body from an OpenAI-compatible API.
/// Shared between embeddings and chat clients.
#[derive(Debug, Deserialize)]
pub struct ApiErrorBody {
    pub error: Option<ApiErrorDetail>,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorDetail {
    pub message: Option<String>,
}

/// Extract a human-readable error message from an API error response body.
pub fn parse_api_error(body_text: &str, status_code: u16) -> String {
    match serde_json::from_str::<ApiErrorBody>(body_text) {
        Ok(err_body) => err_body
            .error
            .and_then(|e| e.message)
            .unwrap_or_else(|| format!("HTTP {}", status_code)),
        Err(_) => format!("HTTP {}", status_code),
    }
}

/// A chunk of transcript text with associated timestamps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub meeting_id: String,
    pub text: String,
    pub start_time: f64,
    pub end_time: f64,
    pub chunk_index: usize,
}

/// Configuration for the RAG/Chat pipeline.
///
/// Embeddings and chat use separate OpenAI-compatible endpoints so that
/// different providers/models can be mixed (e.g. cheap embeddings, powerful chat).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagConfig {
    /// Embeddings endpoint URL (OpenAI-compatible, e.g. OpenRouter).
    pub embeddings_url: String,
    /// API key for the embeddings endpoint.
    pub embeddings_api_key: String,
    /// Model identifier for embeddings (e.g. "openai/text-embedding-3-small").
    pub embeddings_model: String,

    /// Chat/LLM endpoint URL (OpenAI-compatible, e.g. OpenRouter).
    pub chat_url: String,
    /// API key for the chat endpoint.
    pub chat_api_key: String,
    /// Model identifier for chat (e.g. "openai/gpt-4o-mini").
    pub chat_model: String,

    /// Target chunk size in approximate token count (default: 500).
    pub chunk_size: usize,
    /// Number of top-k results to retrieve for context (default: 5).
    pub top_k: usize,
}

impl Default for RagConfig {
    fn default() -> Self {
        Self {
            embeddings_url: String::new(),
            embeddings_api_key: String::new(),
            embeddings_model: String::new(),
            chat_url: String::new(),
            chat_api_key: String::new(),
            chat_model: String::new(),
            chunk_size: 500,
            top_k: 5,
        }
    }
}

impl RagConfig {
    /// Build a `RagConfig` from the application's `RagSettings`.
    pub fn from_settings(settings: &crate::settings::config::RagSettings) -> Self {
        Self {
            embeddings_url: settings.embeddings_url.clone(),
            embeddings_api_key: settings.embeddings_api_key.clone(),
            embeddings_model: settings.embeddings_model.clone(),
            chat_url: settings.chat_url.clone(),
            chat_api_key: settings.chat_api_key.clone(),
            chat_model: settings.chat_model.clone(),
            chunk_size: settings.chunk_size as usize,
            top_k: settings.top_k as usize,
        }
    }

    /// Validate that the configuration has all required URLs set.
    pub fn validate(&self) -> Result<(), String> {
        if self.embeddings_url.is_empty() {
            return Err("Embeddings URL is not configured".to_string());
        }
        if self.chat_url.is_empty() {
            return Err("Chat URL is not configured".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_serialization() {
        let chunk = Chunk {
            id: "c1".into(),
            meeting_id: "m1".into(),
            text: "Hello world".into(),
            start_time: 0.0,
            end_time: 5.0,
            chunk_index: 0,
        };
        let json = serde_json::to_string(&chunk).unwrap();
        let parsed: Chunk = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, "c1");
        assert_eq!(parsed.chunk_index, 0);
    }

    #[test]
    fn test_rag_config_defaults() {
        let config = RagConfig::default();
        assert_eq!(config.chunk_size, 500);
        assert_eq!(config.top_k, 5);
        assert!(config.embeddings_url.is_empty());
    }
}
