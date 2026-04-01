// Glob tool — finds files matching glob patterns.

use std::process::Stdio;

use claude_code_core::tool::{ToolCapabilities, ToolDef, ToolInputSchema};
use claude_code_core::types::DomainError;

use crate::{Tool, ToolContext, ToolOutput};

pub struct GlobTool {
    def: ToolDef,
}

impl GlobTool {
    pub fn new() -> Self {
        Self {
            def: ToolDef {
                name: "Glob".to_string(),
                aliases: vec![],
                search_hint: "Find files matching glob patterns.".to_string(),
                input_schema: ToolInputSchema {
                    schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "pattern": {
                                "type": "string",
                                "description": "Glob pattern to match (e.g. '**/*.rs', 'src/**/*.ts')"
                            },
                            "path": {
                                "type": "string",
                                "description": "Directory to search in"
                            }
                        },
                        "required": ["pattern"]
                    }),
                    strict: false,
                },
                has_output_schema: false,
                capabilities: ToolCapabilities {
                    is_concurrency_safe: true,
                    is_read_only: true,
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
impl Tool for GlobTool {
    fn definition(&self) -> &ToolDef {
        &self.def
    }

    async fn execute(
        &self,
        input: serde_json::Value,
        ctx: &ToolContext,
    ) -> Result<ToolOutput, DomainError> {
        let pattern = input["pattern"]
            .as_str()
            .ok_or_else(|| DomainError::Validation("pattern is required".to_string()))?;

        let search_path = input["path"]
            .as_str()
            .unwrap_or(&ctx.cwd);

        // Use find with -name for simple patterns, or fd if available
        let output = tokio::process::Command::new("find")
            .args([search_path, "-name", pattern, "-type", "f"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| DomainError::Internal(format!("Glob failed: {e}")))?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.trim().is_empty() {
            Ok(ToolOutput {
                content: format!("No files found matching: {pattern}"),
                has_new_messages: false,
                has_modifier: false,
            })
        } else {
            // Sort and limit results
            let mut files: Vec<&str> = stdout.lines().collect();
            files.sort();
            let total = files.len();
            let files: Vec<&str> = files.into_iter().take(200).collect();
            let mut content = files.join("\n");
            if total > 200 {
                content.push_str(&format!("\n... and {} more files", total - 200));
            }
            Ok(ToolOutput {
                content,
                has_new_messages: false,
                has_modifier: false,
            })
        }
    }
}
