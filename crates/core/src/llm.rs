// LLM Provider trait — the core abstraction.
// Plug in any LLM (Claude, OpenAI, Llama, etc.) and the engine works.

use crate::message::SerializedMessage;
use crate::query::{FinishReason, StreamEvent, TokenUsage};
use crate::types::{ContentBlock, DomainError};

// ═══════════════════════════════════════════════
// LLM Request / Response
// ═══════════════════════════════════════════════

/// What we send to the LLM.
#[derive(Debug, Clone)]
pub struct LlmRequest {
    pub system_prompt: String,
    pub messages: Vec<SerializedMessage>,
    pub model: String,
    pub max_tokens: u32,
    pub tools: Vec<LlmToolDef>,
    pub thinking: Option<ThinkingParams>,
}

/// Tool definition sent to the LLM.
#[derive(Debug, Clone)]
pub struct LlmToolDef {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// Thinking/reasoning configuration.
#[derive(Debug, Clone)]
pub struct ThinkingParams {
    pub enabled: bool,
    pub budget_tokens: Option<u32>,
}

/// Final response after stream completes.
#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub content: Vec<ContentBlock>,
    pub finish_reason: FinishReason,
    pub usage: TokenUsage,
    pub model: String,
}

// ═══════════════════════════════════════════════
// Stream types
// ═══════════════════════════════════════════════

/// A boxed stream of LLM events.
pub type LlmStream =
    std::pin::Pin<Box<dyn tokio_stream::Stream<Item = Result<StreamEvent, DomainError>> + Send>>;

// ═══════════════════════════════════════════════
// LLM Provider trait
// ═══════════════════════════════════════════════

/// The core abstraction: any LLM that can produce a stream of events.
///
/// Implement this for Claude, OpenAI, local models, etc.
/// The engine doesn't care which LLM is behind this trait.
///
/// ```ignore
/// // Example: plug in Claude
/// let provider = ClaudeProvider::new(api_key);
/// let engine = QueryEngine::new(provider, tools, config);
/// engine.run("Fix the bug in auth.rs").await?;
///
/// // Example: swap to OpenAI
/// let provider = OpenAiProvider::new(api_key);
/// let engine = QueryEngine::new(provider, tools, config);
/// engine.run("Fix the bug in auth.rs").await?;
/// ```
#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    /// Send a request and get a stream of events back.
    /// This is the primary method — streaming enables responsive UIs.
    async fn stream(&self, request: LlmRequest) -> Result<LlmStream, DomainError>;

    /// Provider name for logging/metrics (e.g., "anthropic", "openai", "ollama").
    fn provider_name(&self) -> &str;

    /// Check if the provider supports a given model ID.
    fn supports_model(&self, model: &str) -> bool;
}
