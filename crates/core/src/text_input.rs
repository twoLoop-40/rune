// @spec Specs/Domain/TextInput.lean ClaudeCode.TextInput

use serde::{Deserialize, Serialize};

use crate::types::AgentId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PromptInputMode {
    Bash,
    Prompt,
    OrphanedPermission,
    TaskNotification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EditablePromptInputMode {
    Bash,
    #[default]
    Prompt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineGhostText {
    pub text: String,
    pub start_col: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanedPermission {
    pub tool_name: String,
    pub input_summary: String,
    pub agent_id: AgentId,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BaseInputState {
    #[serde(default)]
    pub buffer: String,
    #[serde(default)]
    pub cursor_pos: usize,
    pub ghost_text: Option<InlineGhostText>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextInputState {
    #[serde(flatten)]
    pub base: BaseInputState,
    #[serde(default)]
    pub mode: EditablePromptInputMode,
}
