// @spec Specs/Domain/MCPServer.lean ClaudeCode.MCPServer

use serde::{Deserialize, Serialize};

use crate::types::{FilePath, JsonValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConfigScope {
    User,
    Project,
    Plugin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransportType {
    Stdio,
    Sse,
    WebSocket,
    Http,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpStdioConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: Vec<(String, String)>,
    pub cwd: Option<FilePath>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpSseConfig {
    pub url: String,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpWebSocketConfig {
    pub url: String,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpHttpConfig {
    pub url: String,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "transport", rename_all = "camelCase")]
pub enum McpServerConfigVariant {
    Stdio(McpStdioConfig),
    Sse(McpSseConfig),
    WebSocket(McpWebSocketConfig),
    Http(McpHttpConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspServerInstance {
    pub server_name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub language: String,
    #[serde(default)]
    pub is_running: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElicitationWaitingState {
    Idle,
    Waiting { server_name: String },
    Responded,
    TimedOut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElicitationRequestEvent {
    pub server_name: String,
    pub params: JsonValue,
    pub request_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChannelPermissionResponse {
    Granted,
    Denied { reason: String },
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerResource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}
