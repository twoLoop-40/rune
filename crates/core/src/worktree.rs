// @spec Specs/Domain/Worktree.lean ClaudeCode.Worktree

use serde::{Deserialize, Serialize};

use crate::types::{FilePath, SessionId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeSession {
    pub original_cwd: FilePath,
    pub worktree_path: FilePath,
    pub worktree_name: String,
    pub worktree_branch: String,
    pub session_id: SessionId,
    pub tmux_session_name: Option<String>,
    pub creation_dur_ms: Option<u64>,
    #[serde(default)]
    pub used_sparse_paths: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpawnMode {
    SingleSession,
    Worktree,
    SameDir,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedWorktreeSession {
    pub session: WorktreeSession,
    pub task_id: Option<String>,
    #[serde(default = "default_true")]
    pub is_active: bool,
}

fn default_true() -> bool {
    true
}
