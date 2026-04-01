// @spec Specs/Domain/Plugin.lean ClaudeCode.Plugin

use serde::{Deserialize, Serialize};

use crate::types::FilePath;

/// Lean: `structure PluginManifest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub repository: Option<String>,
}

/// Lean: `structure BuiltinPluginDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinPluginDef {
    pub name: String,
    pub description: String,
    pub version: Option<String>,
    #[serde(default = "default_true")]
    pub default_enabled: bool,
}

fn default_true() -> bool {
    true
}

/// Lean: `structure LoadedPluginDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedPluginDef {
    pub name: String,
    pub manifest: PluginManifest,
    pub path: FilePath,
    pub source: String,
    pub repository: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub is_builtin: bool,
    #[serde(default)]
    pub has_commands: bool,
    #[serde(default)]
    pub has_skills: bool,
    #[serde(default)]
    pub has_hooks: bool,
    #[serde(default)]
    pub has_mcp_servers: bool,
    #[serde(default)]
    pub has_lsp_servers: bool,
}

/// Lean: `inductive PluginErrorType` — 21 variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PluginErrorType {
    PathNotFound,
    GitAuthFailed,
    ManifestValidationError,
    PluginNotFound,
    McpConfigInvalid,
    LspServerCrashed,
    MarketplaceBlockedByPolicy,
    DependencyUnsatisfied,
    NetworkError,
    PermissionDenied,
    VersionMismatch,
    InstallFailed,
    LoadFailed,
    HookExecutionFailed,
    SkillNotFound,
    CommandConflict,
    CircularDependency,
    SandboxViolation,
    ConfigParseError,
    TimeoutExceeded,
    RuntimeError,
}

/// Lean: `structure PluginError`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginError {
    pub error_type: PluginErrorType,
    pub source: String,
    pub message: String,
    pub plugin_name: Option<String>,
    pub server_name: Option<String>,
}

/// Lean: `structure McpServerConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: Vec<(String, String)>,
    pub cwd: Option<FilePath>,
}

/// Lean: `structure LspServerConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub language: String,
}

/// Lean: `structure BundledSkillDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundledSkillDef {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub trigger: Vec<String>,
    #[serde(default)]
    pub is_hidden: bool,
}

/// Lean: `inductive PluginLifecycle`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PluginLifecycle {
    Discovered,
    Installing,
    Installed,
    Loading,
    Loaded,
    Enabled,
    Disabled,
    Uninstalling,
    Uninstalled,
    Errored,
}

impl PluginLifecycle {
    /// Valid transitions (from Lean `ValidPluginTransition`).
    /// Any state can transition to Errored.
    pub fn can_transition_to(self, target: Self) -> bool {
        matches!(
            (self, target),
            (Self::Discovered, Self::Installing)
                | (Self::Installing, Self::Installed)
                | (Self::Installed, Self::Loading)
                | (Self::Loading, Self::Loaded)
                | (Self::Loaded, Self::Enabled)
                | (Self::Enabled, Self::Disabled)
                | (Self::Disabled, Self::Enabled)
                | (_, Self::Errored)
        )
    }
}
