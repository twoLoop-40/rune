// @spec Specs/Domain/Bridge.lean ClaudeCode.Bridge

use serde::{Deserialize, Serialize};

use crate::types::{JsonValue, Timestamp};

/// Lean: `inductive BridgeConnectionState`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BridgeConnectionState {
    Disconnected,
    Connecting,
    Connected { session_url: String },
    Error { msg: String },
}

/// Lean: `structure BridgeConfig`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BridgeConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub explicit: bool,
    pub session_url: Option<String>,
}

/// Lean: `inductive BridgeMessageType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BridgeMessageType {
    Prompt,
    Response,
    StatusQuery,
    StatusResponse,
    Interrupt,
    Heartbeat,
}

/// Lean: `structure BridgeMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeMessage {
    #[serde(rename = "type")]
    pub message_type: BridgeMessageType,
    pub payload: JsonValue,
    pub timestamp: Timestamp,
}
