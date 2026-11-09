//! Model provider configuration abstractions.

use serde::{Deserialize, Serialize};

/// Supported model provider backends.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKind {
    OpenAi,
    Gemini,
    Local,
    OfflineMock,
}

/// Typed model provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProviderConfig {
    pub name: String,
    pub kind: ProviderKind,
    pub endpoint_url: Option<String>,
    pub model_id: String,
    pub timeout_secs: u64,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    /// Indicates whether model execution is simulated / offline mock.
    #[serde(default)]
    pub simulated: bool,
}

impl Default for ModelProviderConfig {
    fn default() -> Self {
        Self {
            name: "offline-mock".to_string(),
            kind: ProviderKind::OfflineMock,
            endpoint_url: None,
            model_id: "mock-evaluator".to_string(),
            timeout_secs: 30,
            temperature: 0.0,
            max_tokens: Some(2048),
            simulated: true,
        }
    }
}
