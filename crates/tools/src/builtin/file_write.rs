// FileWrite tool — writes content to files.

use claude_code_core::tool::{ToolCapabilities, ToolDef, ToolInputSchema};
use claude_code_core::types::DomainError;

use crate::{Tool, ToolContext, ToolOutput};

pub struct FileWriteTool {
    def: ToolDef,
}

impl FileWriteTool {
    pub fn new() -> Self {
        Self {
            def: ToolDef {
                name: "FileWrite".to_string(),
                aliases: vec!["Write".to_string()],
                search_hint: "Write content to a file. Creates or overwrites.".to_string(),
                input_schema: ToolInputSchema {
                    schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "file_path": {
                                "type": "string",
                                "description": "Absolute path to the file to write"
                            },
                            "content": {
                                "type": "string",
                                "description": "Content to write to the file"
                            }
                        },
                        "required": ["file_path", "content"]
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
impl Tool for FileWriteTool {
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

        let content = input["content"]
            .as_str()
            .ok_or_else(|| DomainError::Validation("content is required".to_string()))?;

        // Create parent directories if needed
        if let Some(parent) = std::path::Path::new(file_path).parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to create directory: {e}")))?;
        }

        tokio::fs::write(file_path, content)
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to write file: {e}")))?;

        let line_count = content.lines().count();
        Ok(ToolOutput {
            content: format!("Wrote {line_count} lines to {file_path}"),
            has_new_messages: false,
            has_modifier: false,
        })
    }
}
