// @spec Specs/Domain/FileHistory.lean ClaudeCode.FileHistory

use serde::{Deserialize, Serialize};

use crate::types::{AgentId, FilePath, Timestamp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHistoryBackup {
    pub file_path: FilePath,
    pub backup_path: FilePath,
    pub timestamp: Timestamp,
    pub tool_use_id: String,
    pub agent_id: Option<AgentId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHistorySnapshot {
    pub message_id: String,
    pub backups: Vec<FileHistoryBackup>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileHistoryState {
    #[serde(default)]
    pub files: Vec<FileHistoryBackup>,
    #[serde(default)]
    pub snapshots: Vec<FileHistorySnapshot>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffStats {
    pub changed_files: u32,
    pub insertions: u32,
    pub deletions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAttributionState {
    pub file_path: FilePath,
    pub total_chars: u64,
    pub claude_chars: u64,
    pub human_chars: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionSnapshot {
    pub surface: String,
    pub file_states: Vec<FileAttributionState>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributionState {
    #[serde(default)]
    pub files: Vec<FileAttributionState>,
    #[serde(default)]
    pub snapshots: Vec<AttributionSnapshot>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileStateCache {
    #[serde(default)]
    pub entries: Vec<(FilePath, String)>,
    #[serde(default = "default_max")]
    pub max_entries: u32,
}

fn default_max() -> u32 {
    1000
}
