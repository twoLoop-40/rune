// @spec Specs/Domain/Agent.lean ClaudeCode.Agent

use serde::{Deserialize, Serialize};

use crate::types::{AgentId, FilePath, JsonValue, PluginSource};

/// Lean: `inductive AgentSource`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AgentSource {
    BuiltIn,
    Custom,
    Plugin { source: PluginSource },
    Policy,
}

/// Lean: `inductive AgentColorName`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AgentColorName {
    Blue,
    Green,
    Yellow,
    Red,
    Purple,
    Orange,
    Cyan,
    Magenta,
}

/// Lean: `structure AgentMcpServerSpec`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMcpServerSpec {
    pub server_name: String,
    pub config: JsonValue,
}

/// Lean: `structure BaseAgentDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseAgentDef {
    pub name: String,
    pub description: String,
    pub source: AgentSource,
    pub model: Option<String>,
    pub custom_prompt: Option<String>,
    pub append_prompt: Option<String>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub disallowed_tools: Vec<String>,
    #[serde(default)]
    pub mcp_servers: Vec<AgentMcpServerSpec>,
    pub color: Option<AgentColorName>,
}

/// Lean: `inductive AgentDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum AgentDefinition {
    BuiltIn {
        base: BaseAgentDef,
        is_dynamic: bool,
    },
    Custom {
        base: BaseAgentDef,
        config_path: FilePath,
    },
    Plugin {
        base: BaseAgentDef,
        plugin_name: String,
    },
    Policy {
        base: BaseAgentDef,
    },
}

/// Lean: `structure AgentDefinitionsResult`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentDefinitionsResult {
    pub agents: Vec<AgentDefinition>,
    #[serde(default)]
    pub errors: Vec<String>,
}

/// Lean: `structure ForkedAgentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForkedAgentParams {
    pub agent_type: String,
    pub prompt: String,
    pub agent_id: Option<AgentId>,
    pub model: Option<String>,
    pub permission_mode: Option<String>,
    pub isolation: Option<String>,
}

/// Lean: `inductive ForkedAgentResult`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum ForkedAgentResult {
    Success { response: String, agent_id: AgentId },
    Failure { error: String },
}

/// Lean: `structure ResolvedAgent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedAgent {
    pub definition: AgentDefinition,
    pub resolved_model: String,
    pub resolved_tools: Vec<String>,
}

/// Lean: `inductive AgentModelAlias`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AgentModelAlias {
    Sonnet,
    Opus,
    Haiku,
}
