// @spec Specs/Domain/Query.lean ClaudeCode.Query

use serde::{Deserialize, Serialize};

use crate::types::FilePath;

/// Lean: `inductive QuerySource`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QuerySource {
    Repl,
    Sdk,
    Bridge,
    Subagent,
}

/// Lean: `structure ThinkingConfig`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThinkingConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub budget_ms: Option<u64>,
}

fn default_true() -> bool {
    true
}

/// Lean: `structure QueryEngineConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryEngineConfig {
    pub cwd: FilePath,
    pub max_turns: Option<u32>,
    pub max_budget_usd: Option<u64>,
    #[serde(default)]
    pub verbose: bool,
    pub user_specified_model: Option<String>,
    pub fallback_model: Option<String>,
    #[serde(default)]
    pub thinking_config: ThinkingConfig,
    #[serde(default)]
    pub has_json_schema: bool,
}

/// Lean: `inductive FinishReason`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    EndTurn,
    Stop,
    MaxTokens,
    ToolUse,
    Error,
}

/// Lean: `structure TokenUsage`
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    #[serde(default)]
    pub cache_creation: u64,
    #[serde(default)]
    pub cache_read: u64,
}

impl TokenUsage {
    pub fn total(&self) -> u64 {
        self.input_tokens + self.output_tokens
    }
}

/// Lean: `inductive QueryResult` — continue/terminal state machine
#[derive(Debug, Clone)]
pub enum QueryResult {
    Terminal {
        finish_reason: FinishReason,
        usage: TokenUsage,
    },
    Continue {
        next_turn_needed: bool,
    },
}

impl QueryResult {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Terminal { .. })
    }
}

/// Lean: `inductive StreamEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamEvent {
    ContentBlockStart {
        index: u32,
        block_type: String,
        /// For tool_use blocks: the tool use ID
        tool_use_id: Option<String>,
        /// For tool_use blocks: the tool name
        tool_name: Option<String>,
    },
    ContentBlockDelta { index: u32, delta: String },
    ContentBlockStop { index: u32 },
    MessageStart { usage: Option<TokenUsage> },
    MessageStop { finish_reason: FinishReason, usage: Option<TokenUsage> },
}

/// Lean: `inductive CompactionStrategy`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompactionStrategy {
    Summarize,
    Truncate,
    Collapse,
}

/// Lean: `structure QueryTracking`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTracking {
    pub chain_id: String,
    pub depth: u32,
}

pub const MAX_QUERY_DEPTH: u32 = 10;

impl QueryTracking {
    pub fn is_at_max_depth(&self) -> bool {
        self.depth >= MAX_QUERY_DEPTH
    }
}

/// Lean: `inductive ExecutionPhase`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionPhase {
    BuildPrompt,
    NormalizeMessages,
    ApiRequest,
    StreamProcessing,
    ToolExecution,
    ResultCollection,
    ContinueLoop,
    Terminate,
}
