// Grep tool — searches file contents using regex patterns.

use std::path::Path;
use std::process::Stdio;

use claude_code_core::tool::{ToolCapabilities, ToolDef, ToolInputSchema};
use claude_code_core::types::DomainError;

use crate::{Tool, ToolContext, ToolOutput};

pub struct GrepTool {
    def: ToolDef,
}

impl GrepTool {
    pub fn new() -> Self {
        Self {
            def: ToolDef {
                name: "Grep".to_string(),
                aliases: vec![],
                search_hint: "Search file contents using regex patterns (ripgrep).".to_string(),
                input_schema: ToolInputSchema {
                    schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "pattern": {
                                "type": "string",
                                "description": "Regex pattern to search for"
                            },
                            "path": {
                                "type": "string",
                                "description": "Directory or file to search in"
                            },
                            "glob": {
                                "type": "string",
                                "description": "Glob pattern to filter files (e.g. '*.rs')"
                            },
                            "case_insensitive": {
                                "type": "boolean",
                                "description": "Case insensitive search"
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
impl Tool for GrepTool {
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

        let case_insensitive = input["case_insensitive"].as_bool().unwrap_or(false);

        // Try rg first, fall back to grep
        let (cmd, args) = if which_exists("rg") {
            let mut args = vec![
                "-n".to_string(),
                "--max-count=100".to_string(),
                "--max-filesize=1M".to_string(),
            ];
            if case_insensitive {
                args.push("-i".to_string());
            }
            if let Some(glob) = input["glob"].as_str() {
                args.push("--glob".to_string());
                args.push(glob.to_string());
            }
            args.push(pattern.to_string());
            args.push(search_path.to_string());
            ("rg", args)
        } else {
            let mut args = vec!["-rn".to_string()];
            if case_insensitive {
                args.push("-i".to_string());
            }
            args.push(pattern.to_string());
            args.push(search_path.to_string());
            ("grep", args)
        };

        let output = tokio::process::Command::new(cmd)
            .args(&args)
            .current_dir(&ctx.cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| DomainError::Internal(format!("Search failed: {e}")))?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.is_empty() {
            Ok(ToolOutput {
                content: format!("No matches found for pattern: {pattern}"),
                has_new_messages: false,
                has_modifier: false,
            })
        } else {
            let content = if stdout.len() > self.def.max_result_size_chars {
                format!(
                    "{}...\n(truncated)",
                    &stdout[..self.def.max_result_size_chars]
                )
            } else {
                stdout.to_string()
            };
            Ok(ToolOutput {
                content,
                has_new_messages: false,
                has_modifier: false,
            })
        }
    }
}

fn which_exists(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}
