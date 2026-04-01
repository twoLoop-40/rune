// @spec Specs/Domain/Session.lean ClaudeCode.Session

use serde::{Deserialize, Serialize};

use crate::types::{FilePath, SessionId, Timestamp};

/// Lean: `inductive SessionState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionState {
    Idle,
    Running,
    RequiresAction,
}

/// Lean: `inductive SessionActivityType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionActivityType {
    ToolStart,
    Text,
    Result,
    Error,
}

/// Lean: `inductive SessionActivityReason`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionActivityReason {
    ApiCall,
    ToolExec,
}

/// Lean: `structure SessionActivity`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionActivity {
    pub activity_type: SessionActivityType,
    pub reason: SessionActivityReason,
    pub timestamp: Timestamp,
    pub description: Option<String>,
}

/// Lean: `inductive SessionDoneStatus`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionDoneStatus {
    Completed,
    Failed,
    Interrupted,
}

/// Lean: `structure ParsedSessionUrl`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSessionUrl {
    pub host: String,
    pub session_id: String,
    pub token: Option<String>,
}

/// Lean: `structure SessionInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: SessionId,
    pub title: Option<String>,
    pub last_prompt: Option<String>,
    pub timestamp: Timestamp,
    pub cwd: FilePath,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Lean: `structure LiteSessionFile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteSessionFile {
    pub path: FilePath,
    pub session_id: SessionId,
    pub mod_time: Timestamp,
}

/// Lean: `structure SessionExternalMeta`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionExternalMeta {
    pub session_id: SessionId,
    #[serde(default)]
    pub pr_links: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Lean: `structure SessionStorageConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStorageConfig {
    pub storage_path: FilePath,
    #[serde(default = "default_max_sessions")]
    pub max_sessions: u32,
}

fn default_max_sessions() -> u32 {
    1000
}

/// Lean: `structure SessionMemoryConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMemoryConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_scope")]
    pub memory_scope: String,
}

fn default_true() -> bool {
    true
}

fn default_scope() -> String {
    "personal".to_string()
}
