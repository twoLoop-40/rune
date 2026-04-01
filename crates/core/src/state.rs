// @spec Specs/Domain/State.lean ClaudeCode.State

use serde::{Deserialize, Serialize};

use crate::types::{AgentId, Duration, FilePath, JsonValue, ModelSetting, Timestamp};

/// Lean: `inductive McpConnectionStatus`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum McpConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
    Error { msg: String },
}

/// Lean: `structure McpServerConnection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConnection {
    pub server_name: String,
    pub status: McpConnectionStatus,
    #[serde(default)]
    pub tool_count: u32,
}

/// Lean: `structure LoadedPlugin`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedPlugin {
    pub name: String,
    pub path: FilePath,
    pub source: String,
    pub repository: String,
    pub enabled: bool,
    #[serde(default)]
    pub is_builtin: bool,
}

/// Lean: `inductive PluginInstallStatus`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PluginInstallStatus {
    Installed,
    Installing,
    Failed { error: String },
    NotInstalled,
}

/// Lean: `inductive FooterSelection`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FooterSelection {
    Tasks,
    Tmux,
    Bagel,
    Teams,
    Bridge,
}

/// Lean: `inductive ExpandedView`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExpandedView {
    #[default]
    None,
    Tasks,
    Teammates,
}

/// Lean: `structure Notification`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub message: String,
    pub notification_type: String,
    pub timestamp: Timestamp,
}

/// Lean: `structure ElicitationRequest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElicitationRequest {
    pub server_name: String,
    pub params: JsonValue,
}

/// Lean: `structure SpeculationState`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpeculationState {
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub accepted: u32,
    #[serde(default)]
    pub time_saved: Duration,
}

/// Lean: `structure TeammateState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeammateState {
    pub agent_id: AgentId,
    pub name: String,
    #[serde(default)]
    pub is_leader: bool,
}

/// Lean: `structure TeamContext`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamContext {
    pub team_name: String,
    pub lead_agent_id: String,
    pub is_leader: bool,
    #[serde(default)]
    pub teammates: Vec<TeammateState>,
}

/// Lean: `structure InboxMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxMessage {
    pub from_agent_id: AgentId,
    pub content: String,
    pub timestamp: Timestamp,
}

/// Lean: `structure AppState` — DeepImmutable, single source of truth.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    // Settings
    #[serde(default)]
    pub verbose: bool,
    #[serde(default)]
    pub main_loop_model: ModelSetting,
    #[serde(default)]
    pub is_brief_only: bool,

    // MCP
    #[serde(default)]
    pub mcp_connections: Vec<McpServerConnection>,

    // Plugins
    #[serde(default)]
    pub enabled_plugins: Vec<LoadedPlugin>,
    #[serde(default)]
    pub disabled_plugins: Vec<LoadedPlugin>,

    // Tasks
    #[serde(default)]
    pub task_count: u32,
    #[serde(default)]
    pub expanded_view: ExpandedView,
    pub foregrounded_task_id: Option<String>,

    // Team
    pub team_context: Option<TeamContext>,
    #[serde(default)]
    pub inbox: Vec<InboxMessage>,

    // UI
    pub footer_selection: Option<FooterSelection>,
    pub spinner_tip: Option<String>,

    // Speculation
    #[serde(default)]
    pub speculation: SpeculationState,

    // Remote bridge
    #[serde(default)]
    pub repl_bridge_enabled: bool,
    #[serde(default)]
    pub repl_bridge_connected: bool,
    pub repl_bridge_session_url: Option<String>,

    // Auth
    #[serde(default)]
    pub auth_version: u32,
}

impl Default for ModelSetting {
    fn default() -> Self {
        Self::Default
    }
}
