// FileRead tool — reads files and returns content with line numbers.

use claude_code_core::tool::{ToolCapabilities, ToolDef, ToolInputSchema};
use claude_code_core::types::DomainError;

use crate::{Tool, ToolContext, ToolOutput};

pub struct FileReadTool {
    def: ToolDef,
}

impl FileReadTool {
    pub fn new() -> Self {
        Self {
            def: ToolDef {
                name: "FileRead".to_string(),
                aliases: vec!["Read".to_string()],
                search_hint: "Read files from the filesystem. Returns content with line numbers."
                    .to_string(),
                input_schema: ToolInputSchema {
                    schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "file_path": {
                                "type": "string",
                                "description": "Absolute path to the file to read"
                            },
                            "offset": {
                                "type": "integer",
                                "description": "Line number to start reading from (0-based)"
                            },
                            "limit": {
                                "type": "integer",
                                "description": "Number of lines to read"
                            }
                        },
                        "required": ["file_path"]
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
impl Tool for FileReadTool {
    fn definition(&self) -> &ToolDef {
        &self.def
    }

    async fn execute(
        &self,
        input: serde_json::Value,
        _ctx: &ToolContext,
    ) -> Result<ToolOutput, DomainError> {
        let file_path = input["file_path"]
            .as_str()
            .ok_or_else(|| DomainError::Validation("file_path is required".to_string()))?;

        let offset = input["offset"].as_u64().unwrap_or(0) as usize;
        let limit = input["limit"].as_u64().unwrap_or(2000) as usize;

        let content = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| DomainError::NotFound(format!("{file_path}: {e}")))?;

        // Add line numbers (cat -n style)
        let lines: Vec<String> = content
            .lines()
            .skip(offset)
            .take(limit)
            .enumerate()
            .map(|(i, line)| format!("{}\t{}", offset + i + 1, line))
            .collect();

        Ok(ToolOutput {
            content: lines.join("\n"),
            has_new_messages: false,
            has_modifier: false,
        })
    }
}
