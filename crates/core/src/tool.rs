// @spec Specs/Domain/Tool.lean ClaudeCode.Tool

use serde::{Deserialize, Serialize};

use crate::types::JsonValue;

/// Lean: `structure ToolInputSchema`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputSchema {
    pub schema: JsonValue,
    #[serde(default)]
    pub strict: bool,
}

/// Lean: `inductive InterruptBehavior`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InterruptBehavior {
    Cancel,
    Block,
}

/// Lean: `structure SearchReadClassification`
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SearchReadClassification {
    pub is_search: bool,
    pub is_read: bool,
    pub is_list: bool,
}

/// Lean: `structure McpToolInfo`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpToolInfo {
    pub server_name: String,
    pub tool_name: String,
}

/// Lean: `structure ToolCapabilities`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolCapabilities {
    #[serde(default)]
    pub is_concurrency_safe: bool,
    #[serde(default)]
    pub is_read_only: bool,
    #[serde(default)]
    pub is_destructive: bool,
    #[serde(default = "default_true")]
    pub is_enabled: bool,
    #[serde(default)]
    pub should_defer: bool,
    #[serde(default)]
    pub always_load: bool,
}

fn default_true() -> bool {
    true
}

impl ToolCapabilities {
    /// Lean theorem: destructive → !read_only (runtime assertion)
    pub fn validate(&self) -> bool {
        if self.is_destructive {
            !self.is_read_only
        } else {
            true
        }
    }
}

/// Lean: `structure ToolDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub search_hint: String,
    pub input_schema: ToolInputSchema,
    #[serde(default)]
    pub has_output_schema: bool,
    #[serde(default)]
    pub capabilities: ToolCapabilities,
    #[serde(default = "default_interrupt")]
    pub interrupt_behavior: InterruptBehavior,
    #[serde(default)]
    pub is_mcp: bool,
    #[serde(default)]
    pub is_lsp: bool,
    pub mcp_info: Option<McpToolInfo>,
    #[serde(default = "default_max_result_size")]
    pub max_result_size_chars: usize,
}

fn default_interrupt() -> InterruptBehavior {
    InterruptBehavior::Cancel
}

fn default_max_result_size() -> usize {
    200_000
}

/// Lean: `inductive BuiltinToolName`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BuiltinToolName {
    // File system
    Bash,
    FileRead,
    FileEdit,
    FileWrite,
    Glob,
    Grep,
    // Agent
    Agent,
    SendMessage,
    TaskCreate,
    TaskStop,
    TaskOutput,
    TeamCreate,
    TeamDelete,
    // Web
    WebSearch,
    WebFetch,
    // UI
    AskUser,
    TodoWrite,
    NotebookEdit,
    // Plan
    EnterPlanMode,
    ExitPlanMode,
    // Worktree
    EnterWorktree,
    ExitWorktree,
    // Remote
    RemoteTrigger,
    CronCreate,
    CronDelete,
    CronList,
    // MCP (dynamic)
    Mcp {
        server_name: String,
        tool_name: String,
    },
}

impl BuiltinToolName {
    pub fn is_read_only(&self) -> bool {
        matches!(
            self,
            Self::FileRead | Self::Glob | Self::Grep | Self::WebSearch | Self::WebFetch
        )
    }

    pub fn is_concurrency_safe(&self) -> bool {
        matches!(
            self,
            Self::FileRead
                | Self::Glob
                | Self::Grep
                | Self::WebSearch
                | Self::WebFetch
                | Self::TodoWrite
        )
    }
}
