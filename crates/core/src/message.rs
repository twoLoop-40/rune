// @spec Specs/Domain/Message.lean ClaudeCode.Message

use serde::{Deserialize, Serialize};

use crate::types::{AgentId, ContentBlock, FilePath, SessionId, Timestamp, Uuid};

/// Lean: `inductive MessageRole`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Lean: `inductive MessageType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
    User,
    Assistant,
    System,
    Attachment,
    Progress,
    Tombstone,
}

/// Lean: `inductive MessageOrigin`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageOrigin {
    UserTyped,
    SlashCommand,
    Queue,
    Hook,
    Subagent,
    Bridge,
    Sdk,
}

/// Lean: `structure SerializedMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedMessage {
    pub uuid: Uuid,
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub role: MessageRole,
    pub content: Vec<ContentBlock>,
    pub cwd: FilePath,
    pub session_id: SessionId,
    pub timestamp: Timestamp,
    pub version: u32,
}

/// Lean: `structure TranscriptMessage extends SerializedMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptMessage {
    #[serde(flatten)]
    pub base: SerializedMessage,
    pub parent_uuid: Option<Uuid>,
    #[serde(default)]
    pub is_sidechain: bool,
    pub agent_id: Option<AgentId>,
}

/// Lean: `inductive MetadataType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MetadataType {
    Summary,
    CustomTitle,
    AiTitle,
    LastPrompt,
    TaskSummary,
    Tag,
    AgentName,
    AgentColor,
    AgentSetting,
    PrLink,
    FileHistorySnapshot,
    AttributionSnapshot,
    SpeculationAccept,
    ContextCollapseCommit,
    ContextCollapseSnapshot,
}

/// Lean: `structure PRLinkMeta`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrLinkMeta {
    pub pr_number: u32,
    pub pr_url: String,
    pub pr_repository: String,
}

/// Lean: `inductive QueuePriority`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QueuePriority {
    Now,
    Next,
    Later,
}

/// Lean: `inductive QueueMode`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QueueMode {
    Bash,
    Prompt,
    OrphanedPermission,
    TaskNotification,
}

/// Lean: `structure QueuedCommand`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedCommand {
    pub value: String,
    pub mode: QueueMode,
    pub priority: QueuePriority,
    pub origin: MessageOrigin,
    pub agent_id: AgentId,
}

/// Lean: `inductive NormalizationStep`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationStep {
    ToApi,
    FromApi,
    ToTranscript,
}

impl SerializedMessage {
    /// Lean: `MessageInvariant` — runtime assertion
    pub fn validate(&self) -> bool {
        !self.uuid.0.is_empty() && self.version > 0
    }
}
