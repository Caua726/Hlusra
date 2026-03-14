use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::rag::types::{parse_api_error, RagConfig};

/// Request body for the OpenAI-compatible embeddings endpoint.
#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    model: String,
    input: Vec<String>,
}

/// A single embedding object returned by the API.
#[derive(Debug, Deserialize)]
struct EmbeddingObject {
    #[allow(dead_code)]
    index: usize,
    embedding: Vec<f32>,
}

/// Top-level response from the embeddings endpoint.
#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingObject>,
    #[allow(dead_code)]
    model: Option<String>,
    #[allow(dead_code)]
    usage: Option<serde_json::Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum EmbeddingsError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },
    #[error("Embedding dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },
    #[error("No embeddings returned from API")]
    EmptyResponse,
}

impl serde::Serialize for EmbeddingsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// HTTP client for OpenAI-compatible embeddings endpoints (focused on OpenRouter).
pub struct EmbeddingsClient {
    client: Client,
    url: String,
    api_key: String,
    model: String,
}

impl EmbeddingsClient {
    /// Create a new client from the RAG configuration.
    pub fn new(config: &RagConfig) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .unwrap_or_else(|_| Client::new()),
            url: config.embeddings_url.clone(),
            api_key: config.embeddings_api_key.clone(),
            model: config.embeddings_model.clone(),
        }
    }

    /// Returns the model name this client is configured to use.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Embed a single text string.
    pub async fn embed_one(&self, text: &str) -> Result<Vec<f32>, EmbeddingsError> {
        let results = self.embed_batch(&[text.to_string()]).await?;
        results
            .into_iter()
            .next()
            .ok_or(EmbeddingsError::EmptyResponse)
    }

    /// Embed a batch of text strings.
    ///
    /// Returns one `Vec<f32>` per input, in the same order.
    pub async fn embed_batch(
        &self,
        texts: &[String],
    ) -> Result<Vec<Vec<f32>>, EmbeddingsError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let body = EmbeddingRequest {
            model: self.model.clone(),
            input: texts.to_vec(),
        };

        let url = format!("{}/embeddings", self.url.trim_end_matches('/'));

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let status_code = status.as_u16();
            let body_text = response.text().await.unwrap_or_default();
            let message = parse_api_error(&body_text, status_code);
            return Err(EmbeddingsError::Api {
                status: status_code,
                message,
            });
        }

        let resp: EmbeddingResponse = response.json().await?;
        if resp.data.is_empty() {
            return Err(EmbeddingsError::EmptyResponse);
        }

        // Validate that we got the expected number of embeddings back.
        if resp.data.len() != texts.len() {
            return Err(EmbeddingsError::DimensionMismatch {
                expected: texts.len(),
                got: resp.data.len(),
            });
        }

        // Sort by index to guarantee order matches input.
        let mut data = resp.data;
        data.sort_by_key(|e| e.index);

        // Validate all embeddings have the same dimension.
        let dim = data[0].embedding.len();
        for obj in &data {
            if obj.embedding.len() != dim {
                return Err(EmbeddingsError::DimensionMismatch {
                    expected: dim,
                    got: obj.embedding.len(),
                });
            }
        }

        Ok(data.into_iter().map(|e| e.embedding).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_request_serialization() {
        let req = EmbeddingRequest {
            model: "test-model".into(),
            input: vec!["hello".into(), "world".into()],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test-model"));
        assert!(json.contains("hello"));
    }

    #[test]
    fn test_embedding_response_deserialization() {
        let json = r#"{
            "data": [
                {"index": 0, "embedding": [0.1, 0.2, 0.3]},
                {"index": 1, "embedding": [0.4, 0.5, 0.6]}
            ],
            "model": "test",
            "usage": null
        }"#;
        let resp: EmbeddingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[0].embedding.len(), 3);
    }
}
