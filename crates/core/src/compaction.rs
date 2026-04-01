// @spec Specs/Domain/Compaction.lean ClaudeCode.Compaction

use serde::{Deserialize, Serialize};

use crate::types::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompactionStrategy {
    Summarize,
    Truncate,
    Collapse,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeBasedMcConfig {
    #[serde(default = "default_interval")]
    pub interval_ms: u64,
    #[serde(default = "default_max_messages")]
    pub max_messages: u32,
}

fn default_interval() -> u64 {
    300_000
}

fn default_max_messages() -> u32 {
    100
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoCompactTrackingState {
    pub last_compact_time: Option<Timestamp>,
    #[serde(default)]
    pub messages_since: u32,
    #[serde(default)]
    pub compaction_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecompactionInfo {
    pub original_size: u64,
    pub compacted_size: u64,
    pub removed_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContextEditStrategy {
    Replace,
    Append,
    Prepend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextManagementConfig {
    #[serde(default = "default_max_tokens")]
    pub max_context_tokens: u32,
    #[serde(default = "default_threshold")]
    pub compact_threshold: f64,
    #[serde(default = "default_strategy")]
    pub strategy: CompactionStrategy,
}

fn default_max_tokens() -> u32 {
    200_000
}

fn default_threshold() -> f64 {
    0.8
}

fn default_strategy() -> CompactionStrategy {
    CompactionStrategy::Summarize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingCacheEdits {
    pub edits: Vec<(String, String)>,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrocompactResult {
    pub removed_tokens: u64,
    pub removed_messages: u32,
    pub strategy: CompactionStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCollapseCommitEntry {
    pub collapse_id: String,
    pub summary: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContextCollapseSnapshotEntry {
    #[serde(default)]
    pub staged: Vec<String>,
    #[serde(default)]
    pub armed: Vec<String>,
}
