// @spec Specs/Domain/Permission.lean ClaudeCode.Permission
//
// Permission system — modes, decisions, classifier, denial tracking.

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════
// Permission Mode
// ═══════════════════════════════════════════════

/// Lean: `inductive PermissionMode`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionMode {
    Default,
    AcceptEdits,
    BypassPermissions,
    DontAsk,
    Plan,
    Auto,
    /// Internal: delegate to parent.
    Bubble,
}

// ═══════════════════════════════════════════════
// Permission Behavior & Rule
// ═══════════════════════════════════════════════

/// Lean: `inductive PermissionBehavior`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionBehavior {
    Allow,
    Deny,
    Ask,
    Passthrough,
}

/// Lean: `inductive PermissionRuleSource`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionRuleSource {
    UserSettings,
    ProjectSettings,
    CliFlag,
    Hook,
    Classifier,
    PluginPolicy,
}

/// Lean: `structure PermissionRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRule {
    pub source: PermissionRuleSource,
    pub rule_behavior: PermissionBehavior,
    pub tool_name: String,
    pub rule_content: Option<String>,
}

// ═══════════════════════════════════════════════
// Permission Decision
// ═══════════════════════════════════════════════

/// Lean: `inductive DenyReason`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DenyReason {
    BlockedByPolicy,
    BlockedByHook { msg: String },
    BlockedByClassifier { reason: String },
    UserDenied,
    PlanModeRestriction,
}

/// Lean: `inductive PermissionDecision`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PermissionDecision {
    Allow {
        #[serde(default)]
        user_modified: bool,
    },
    Ask {
        message: String,
        blocked_path: Option<String>,
    },
    Deny {
        reason: DenyReason,
        message: String,
    },
    Passthrough {
        message: String,
    },
}

impl PermissionDecision {
    /// Lean: `def PermissionDecision.isAllowed`
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allow { .. })
    }

    /// Lean: `def PermissionDecision.isDenied`
    pub fn is_denied(&self) -> bool {
        matches!(self, Self::Deny { .. })
    }
}

/// Lean theorem: `bypass_always_allows`
/// In bypass mode, all tool calls are allowed.
pub fn check_bypass(mode: PermissionMode) -> Option<PermissionDecision> {
    if mode == PermissionMode::BypassPermissions {
        Some(PermissionDecision::Allow {
            user_modified: false,
        })
    } else {
        None
    }
}

// ═══════════════════════════════════════════════
// Classifier
// ═══════════════════════════════════════════════

/// Lean: `inductive ClassifierStage`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClassifierStage {
    Fast,
    Thinking,
}

/// Lean: `structure ClassifierResult`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifierResult {
    pub should_block: bool,
    pub reason: String,
    #[serde(default)]
    pub unavailable: bool,
    #[serde(default)]
    pub transcript_too_long: bool,
    pub model: String,
    pub stage: Option<ClassifierStage>,
    pub duration_ms: Option<u64>,
}

// ═══════════════════════════════════════════════
// Permission Flow
// ═══════════════════════════════════════════════

/// Lean: `inductive PermissionFlowStep`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionFlowStep {
    ValidateInput,
    CheckPermissions,
    ExecuteAllowed,
    PromptUser,
    RunClassifier,
    RejectDenied,
    DelegateToHook,
}

// ═══════════════════════════════════════════════
// Denial Tracking
// ═══════════════════════════════════════════════

/// Lean: `structure DenialTrackingState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenialTrackingState {
    pub consecutive_denials: u32,
    #[serde(default = "default_threshold")]
    pub threshold: u32,
    #[serde(default)]
    pub fallback_to_ask: bool,
}

fn default_threshold() -> u32 {
    3
}

impl DenialTrackingState {
    /// Lean: `def DenialTrackingState.shouldFallback`
    pub fn should_fallback(&self) -> bool {
        self.consecutive_denials >= self.threshold
    }
}
