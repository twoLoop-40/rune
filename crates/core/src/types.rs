// @spec Specs/Common/Types.lean ClaudeCode
//
// Foundation types shared across all domain modules.
// Lean 4: structure → Rust struct, inductive → Rust enum

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════
// Branded ID Types (compile-time safety)
// ═══════════════════════════════════════════════

/// Session identifier. Created by `get_session_id()`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

/// Agent identifier. Pattern: /^a(?:.+-)?[0-9a-f]{16}$/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub String);

/// UUID string wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uuid(pub String);

// ═══════════════════════════════════════════════
// Timestamp & Duration
// ═══════════════════════════════════════════════

/// Millisecond-precision timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp {
    pub ms: u64,
}

/// Millisecond-precision duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Duration {
    pub ms: u64,
}

// ═══════════════════════════════════════════════
// Result / Error
// ═══════════════════════════════════════════════

/// Domain error types.
/// Lean: `inductive DomainError`
#[derive(Debug, Clone, thiserror::Error)]
pub enum DomainError {
    #[error("validation: {0}")]
    Validation(String),
    #[error("permission: {0}")]
    Permission(String),
    #[error("timeout: {0}")]
    Timeout(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("internal: {0}")]
    Internal(String),
}

/// Domain result alias.
pub type DomainResult<T> = Result<T, DomainError>;

// ═══════════════════════════════════════════════
// File System Primitives
// ═══════════════════════════════════════════════

/// Absolute file path.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FilePath(pub String);

/// Glob pattern.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlobPattern(pub String);

// ═══════════════════════════════════════════════
// JSON-like Value
// ═══════════════════════════════════════════════

// We re-export serde_json::Value as our JsonValue
pub use serde_json::Value as JsonValue;

// ═══════════════════════════════════════════════
// Source / Origin
// ═══════════════════════════════════════════════

/// Setting source.
/// Lean: `inductive SettingSource`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SettingSource {
    UserSettings,
    ProjectSettings,
    EnvVar,
    CliFlag,
    PluginConfig,
}

/// Plugin source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginSource {
    pub name: String,
    pub repository: String,
}

// ═══════════════════════════════════════════════
// Content Block (API protocol)
// ═══════════════════════════════════════════════

/// Anthropic API ContentBlockParam abstraction.
/// Lean: `inductive ContentBlock`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text {
        content: String,
    },
    Image {
        media_type: String,
        data: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: JsonValue,
    },
    ToolResult {
        tool_use_id: String,
        content: String,
        is_error: bool,
    },
    Thinking {
        content: String,
    },
}

// ═══════════════════════════════════════════════
// Model & Effort
// ═══════════════════════════════════════════════

/// Model setting.
/// Lean: `inductive ModelSetting`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelSetting {
    Default,
    Named { model_id: String },
}

/// Reasoning effort level.
/// Lean: `inductive EffortValue`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EffortValue {
    Low,
    Medium,
    High,
}
