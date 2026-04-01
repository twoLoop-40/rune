// @spec Specs/Domain/RemoteSession.lean ClaudeCode.RemoteSession

use serde::{Deserialize, Serialize};

use crate::types::Timestamp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RemoteTaskType {
    Standard,
    AutofixPr,
    CodeReview,
    Scheduled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RemoteAgentPreconditionResult {
    Ready,
    NeedsAuth { message: String },
    NeedsSetup { message: String },
    Unavailable { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutofixPrMeta {
    pub pr_number: u32,
    pub pr_url: String,
    pub repository: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteAgentTaskState {
    pub task_id: String,
    pub remote_type: RemoteTaskType,
    pub session_url: Option<String>,
    #[serde(default = "default_poll_interval")]
    pub poll_interval_ms: u64,
}

fn default_poll_interval() -> u64 {
    5000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundRemoteSession {
    pub session_id: String,
    pub task_id: String,
    pub status: String,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SessionContextSource {
    Git { repo: String, branch: String },
    KnowledgeBase { id: String },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionContext {
    pub sources: Vec<SessionContextSource>,
    #[serde(default)]
    pub resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollRemoteSessionResponse {
    pub status: String,
    pub is_terminal: bool,
    pub output_url: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSession {
    pub session_id: String,
    pub status: String,
    pub created_at: Timestamp,
    pub updated_at: Option<Timestamp>,
    pub title: Option<String>,
    pub context: Option<SessionContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteTriggerConfig {
    pub schedule: String,
    pub prompt: String,
    pub model: Option<String>,
    pub max_budget: Option<u64>,
}
