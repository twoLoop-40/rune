// @spec Specs/Domain/Model.lean ClaudeCode.Model

use serde::{Deserialize, Serialize};

use crate::types::Timestamp;

/// Lean: `inductive APIProvider`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiProvider {
    Anthropic,
    Bedrock,
    Vertex,
    Custom { endpoint: String },
}

/// Lean: `inductive ModelAlias`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModelAlias {
    Sonnet,
    Opus,
    Haiku,
}

/// Lean: `inductive ModelCapability`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ModelCapability {
    Thinking,
    Vision,
    ToolUse,
    Streaming,
    ExtendedOutput,
    ComputerUse,
}

/// Lean: `structure CanonicalModelId`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CanonicalModelId(pub String);

/// Lean: `structure ModelConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: ApiProvider,
    pub model_id: CanonicalModelId,
    pub alias: Option<ModelAlias>,
    #[serde(default)]
    pub capabilities: Vec<ModelCapability>,
    #[serde(default = "default_max_output")]
    pub max_output_tokens: u32,
    #[serde(default = "default_context_window")]
    pub context_window: u32,
}

fn default_max_output() -> u32 {
    16384
}

fn default_context_window() -> u32 {
    200_000
}

/// Lean: `structure ModelCosts` — per million tokens, in microdollars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelCosts {
    pub input_tokens: u64,
    pub output_tokens: u64,
    #[serde(default)]
    pub cache_creation: u64,
    #[serde(default)]
    pub cache_read: u64,
    #[serde(default)]
    pub web_search: u64,
}

/// Lean: `structure ModelStrings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStrings {
    pub display_name: String,
    pub short_name: String,
    #[serde(default)]
    pub description: String,
}

/// Lean: `structure ModelOverrideConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelOverrideConfig {
    pub model_id: String,
    pub reason: String,
}

/// Lean: `structure ModelOption`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelOption {
    pub model_id: CanonicalModelId,
    pub display_name: String,
    #[serde(default)]
    pub is_default: bool,
    #[serde(default = "default_true")]
    pub is_available: bool,
}

fn default_true() -> bool {
    true
}

/// Lean: `structure BedrockRegionPrefix`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedrockRegionPrefix {
    pub region: String,
    pub region_prefix: String,
}

/// Lean: `structure CostTrackerState`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CostTrackerState {
    #[serde(default)]
    pub total_input_tokens: u64,
    #[serde(default)]
    pub total_output_tokens: u64,
    #[serde(default)]
    pub total_cache_creation: u64,
    #[serde(default)]
    pub total_cache_read: u64,
    #[serde(default)]
    pub total_cost_usd: u64,
    #[serde(default)]
    pub turn_count: u32,
}

/// Lean: `structure ApiMetricsEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMetricsEntry {
    pub ttft_ms: u64,
    pub model: String,
    pub timestamp: Timestamp,
}
