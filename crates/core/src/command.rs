// @spec Specs/Domain/Command.lean ClaudeCode.Command

use serde::{Deserialize, Serialize};

use crate::types::{EffortValue, GlobPattern, SettingSource};

/// Lean: `inductive CommandType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommandType {
    Prompt,
    Local,
    LocalJsx,
}

/// Lean: `inductive CommandAvailability`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommandAvailability {
    ClaudeAi,
    Console,
}

/// Lean: `inductive PromptContext`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PromptContext {
    Inline,
    Fork,
}

/// Lean: `structure CommandBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandBase {
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub description: String,
    #[serde(default = "default_availability")]
    pub availability: Vec<CommandAvailability>,
    #[serde(default = "default_true")]
    pub is_enabled: bool,
    #[serde(default)]
    pub is_hidden: bool,
    #[serde(default)]
    pub is_mcp: bool,
    #[serde(default)]
    pub argument_hint: String,
    #[serde(default)]
    pub when_to_use: String,
    #[serde(default)]
    pub version: String,
    #[serde(default = "default_true")]
    pub user_invocable: bool,
    #[serde(default)]
    pub immediate: bool,
    #[serde(default)]
    pub is_sensitive: bool,
    pub kind: Option<String>,
}

fn default_availability() -> Vec<CommandAvailability> {
    vec![CommandAvailability::Console]
}

fn default_true() -> bool {
    true
}

/// Lean: `structure PromptCommandDef extends CommandBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptCommandDef {
    #[serde(flatten)]
    pub base: CommandBase,
    #[serde(default = "default_prompt_type")]
    pub command_type: CommandType,
    #[serde(default)]
    pub progress_message: String,
    #[serde(default)]
    pub content_length: u32,
    #[serde(default)]
    pub arg_names: Vec<String>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    pub model: Option<String>,
    #[serde(default = "default_source")]
    pub source: SettingSource,
    #[serde(default = "default_context")]
    pub context: PromptContext,
    #[serde(default)]
    pub agent: String,
    #[serde(default = "default_effort")]
    pub effort: EffortValue,
    #[serde(default)]
    pub paths: Vec<GlobPattern>,
}

fn default_prompt_type() -> CommandType {
    CommandType::Prompt
}

fn default_source() -> SettingSource {
    SettingSource::UserSettings
}

fn default_context() -> PromptContext {
    PromptContext::Inline
}

fn default_effort() -> EffortValue {
    EffortValue::Medium
}

/// Lean: `structure LocalCommandDef extends CommandBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalCommandDef {
    #[serde(flatten)]
    pub base: CommandBase,
    #[serde(default = "default_local_type")]
    pub command_type: CommandType,
    #[serde(default)]
    pub supports_non_interactive: bool,
}

fn default_local_type() -> CommandType {
    CommandType::Local
}

/// Lean: `inductive CommandCategory`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommandCategory {
    Session,
    Git,
    Config,
    Agent,
    Navigation,
    Help,
    Skill,
    Debug,
    Mcp,
    Plugin,
}

/// Lean: `structure CommandRegistry`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommandRegistry {
    pub builtin_commands: Vec<CommandBase>,
    #[serde(default)]
    pub plugin_commands: Vec<CommandBase>,
    #[serde(default)]
    pub mcp_commands: Vec<CommandBase>,
}

impl CommandRegistry {
    /// Lean: `def CommandRegistry.findByName`
    pub fn find_by_name(&self, name: &str) -> Option<&CommandBase> {
        self.all_commands()
            .find(|cmd| cmd.name == name || cmd.aliases.contains(&name.to_string()))
    }

    fn all_commands(&self) -> impl Iterator<Item = &CommandBase> {
        self.builtin_commands
            .iter()
            .chain(&self.plugin_commands)
            .chain(&self.mcp_commands)
    }
}
