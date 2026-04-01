// @spec Specs/Domain/Hook.lean ClaudeCode.Hook

use serde::{Deserialize, Serialize};

/// Lean: `inductive HookEvent` — 15 variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HookEvent {
    SessionStart,
    Setup,
    SubagentStart,
    PreToolUse,
    PostToolUse,
    PostToolUseFailure,
    UserPromptSubmit,
    FileChanged,
    CwdChanged,
    PermissionRequest,
    PermissionDenied,
    Elicitation,
    ElicitationResult,
    Notification,
    WorktreeCreate,
}

/// Lean: `structure HookCallback`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookCallback {
    pub timeout: Option<u32>,
    #[serde(default)]
    pub internal: bool,
}

/// Lean: `inductive HookDecision`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum HookDecision {
    Approve,
    Block { reason: String },
}

/// Lean: `structure HookOutput`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HookOutput {
    #[serde(default = "default_true")]
    pub continue_: bool,
    #[serde(default)]
    pub suppress_output: bool,
    pub stop_reason: Option<String>,
    pub decision: Option<HookDecision>,
    pub system_message: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Lean: `inductive HookOutcome`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HookOutcome {
    Success,
    Blocking { error: String },
    NonBlockingError { error: String },
    Cancelled,
}

/// Lean: `structure HookResult`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookResult {
    pub outcome: HookOutcome,
    #[serde(default)]
    pub prevent_continuation: bool,
    pub stop_reason: Option<String>,
    pub permission_behavior: Option<String>,
    pub additional_context: Option<String>,
}

/// Lean: `structure HookRegistration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookRegistration {
    pub event: HookEvent,
    pub matcher: Option<String>,
    pub callback: HookCallback,
    #[serde(default = "default_priority")]
    pub priority: u32,
}

fn default_priority() -> u32 {
    100
}
