// Bash tool — executes shell commands and returns output.

use claude_code_core::tool::{ToolCapabilities, ToolDef, ToolInputSchema};
use claude_code_core::types::DomainError;

use crate::{Tool, ToolContext, ToolOutput};

pub struct BashTool {
    def: ToolDef,
}

impl BashTool {
    pub fn new() -> Self {
        Self {
            def: ToolDef {
                name: "Bash".to_string(),
                aliases: vec![],
                search_hint: "Execute shell commands and return output.".to_string(),
                input_schema: ToolInputSchema {
                    schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "command": {
                                "type": "string",
                                "description": "The shell command to execute"
                            },
                            "timeout": {
                                "type": "integer",
                                "description": "Timeout in milliseconds (default: 120000)"
                            }
                        },
                        "required": ["command"]
                    }),
                    strict: false,
                },
                has_output_schema: false,
                capabilities: ToolCapabilities {
                    is_concurrency_safe: false,
                    is_read_only: false,
                    is_destructive: false,
                    is_enabled: true,
                    should_defer: false,
                    always_load: true,
                },
                interrupt_behavior: claude_code_core::tool::InterruptBehavior::Cancel,
                is_mcp: false,
                is_lsp: false,
                mcp_info: None,
                max_result_size_chars: 200_000,
            },
        }
    }
}

#[async_trait::async_trait]
impl Tool for BashTool {
    fn definition(&self) -> &ToolDef {
        &self.def
    }

    async fn execute(
        &self,
        input: serde_json::Value,
        ctx: &ToolContext,
    ) -> Result<ToolOutput, DomainError> {
        let command = input["command"]
            .as_str()
            .ok_or_else(|| DomainError::Validation("command is required".to_string()))?;

        let timeout_ms = input["timeout"].as_u64().unwrap_or(120_000);

        let result = tokio::time::timeout(
            std::time::Duration::from_millis(timeout_ms),
            tokio::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(&ctx.cwd)
                .output(),
        )
        .await;

        match result {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let exit_code = output.status.code().unwrap_or(-1);

                let content = if exit_code == 0 {
                    if stderr.is_empty() {
                        stdout.to_string()
                    } else {
                        format!("{stdout}\n(stderr: {stderr})")
                    }
                } else {
                    format!("Exit code {exit_code}\nstdout: {stdout}\nstderr: {stderr}")
                };

                // Truncate if too large
                let content = if content.len() > self.def.max_result_size_chars {
                    format!(
                        "{}...\n(truncated, {} chars total)",
                        &content[..self.def.max_result_size_chars],
                        content.len()
                    )
                } else {
                    content
                };

                Ok(ToolOutput {
                    content,
                    has_new_messages: false,
                    has_modifier: false,
                })
            }
            Ok(Err(e)) => Err(DomainError::Internal(format!("Command failed: {e}"))),
            Err(_) => Err(DomainError::Timeout(format!(
                "Command timed out after {timeout_ms}ms"
            ))),
        }
    }
}
