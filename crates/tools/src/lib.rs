// @spec Specs/Domain/Tool.lean ClaudeCode.Tool
//
// Tool trait — the extension point for all tool implementations.
// Built-in tools (Bash, FileRead, etc.) and MCP tools all implement this.

pub mod registry;
pub mod builtin;

pub use registry::ToolRegistry;

use claude_code_core::hook::HookDecision;
use claude_code_core::permission::PermissionDecision;
use claude_code_core::tool::{ToolCapabilities, ToolDef};
use claude_code_core::types::DomainError;

// ═══════════════════════════════════════════════
// Tool trait
// ═══════════════════════════════════════════════

/// The result of executing a tool.
#[derive(Debug, Clone)]
pub struct ToolOutput {
    /// The content to return to the LLM.
    pub content: String,
    /// Whether this tool produced new messages (e.g., subagent).
    pub has_new_messages: bool,
    /// Whether a modifier key was held (UI hint).
    pub has_modifier: bool,
}

/// Context passed to tools during execution.
#[derive(Debug, Clone)]
pub struct ToolContext {
    /// Current working directory.
    pub cwd: String,
    /// Agent ID if running inside a subagent.
    pub agent_id: Option<String>,
    /// Tool use ID from the LLM.
    pub tool_use_id: String,
}

/// The core tool trait. Every tool (built-in, MCP, plugin) implements this.
///
/// Lean: `structure ToolDef` + execution methods
///
/// ```ignore
/// struct BashTool;
///
/// #[async_trait]
/// impl Tool for BashTool {
///     fn definition(&self) -> ToolDef { ... }
///     async fn execute(&self, input: Value, ctx: &ToolContext) -> Result<ToolOutput, DomainError> {
///         let command = input["command"].as_str().unwrap();
///         let output = tokio::process::Command::new("sh")
///             .arg("-c").arg(command)
///             .output().await?;
///         Ok(ToolOutput { content: String::from_utf8_lossy(&output.stdout).into(), .. })
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    /// Tool definition (name, schema, capabilities).
    fn definition(&self) -> &ToolDef;

    /// Execute the tool with the given JSON input.
    async fn execute(
        &self,
        input: serde_json::Value,
        ctx: &ToolContext,
    ) -> Result<ToolOutput, DomainError>;

    /// Validate input before execution. Default: always valid.
    fn validate_input(&self, _input: &serde_json::Value) -> Result<(), DomainError> {
        Ok(())
    }

    /// Get capabilities (delegates to definition by default).
    fn capabilities(&self) -> &ToolCapabilities {
        &self.definition().capabilities
    }
}

// ═══════════════════════════════════════════════
// Permission checker trait
// ═══════════════════════════════════════════════

/// Checks whether a tool call is permitted.
/// Lean: Permission.lean PermissionDecision
#[async_trait::async_trait]
pub trait PermissionChecker: Send + Sync {
    async fn check(
        &self,
        tool_name: &str,
        input: &serde_json::Value,
        ctx: &ToolContext,
    ) -> PermissionDecision;
}

// ═══════════════════════════════════════════════
// Hook runner trait
// ═══════════════════════════════════════════════

/// Runs hooks before/after tool execution.
/// Lean: Hook.lean HookEvent → HookResult
#[async_trait::async_trait]
pub trait HookRunner: Send + Sync {
    /// Run pre-tool-use hooks. Returns Block to prevent execution.
    async fn pre_tool_use(
        &self,
        tool_name: &str,
        input: &serde_json::Value,
    ) -> Option<HookDecision>;

    /// Run post-tool-use hooks.
    async fn post_tool_use(
        &self,
        tool_name: &str,
        input: &serde_json::Value,
        output: &ToolOutput,
    );
}
