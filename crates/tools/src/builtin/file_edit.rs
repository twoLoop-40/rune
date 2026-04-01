// FileEdit tool — performs exact string replacements in files.

use claude_code_core::tool::{ToolCapabilities, ToolDef, ToolInputSchema};
use claude_code_core::types::DomainError;

use crate::{Tool, ToolContext, ToolOutput};

pub struct FileEditTool {
    def: ToolDef,
}

impl FileEditTool {
    pub fn new() -> Self {
        Self {
            def: ToolDef {
                name: "FileEdit".to_string(),
                aliases: vec!["Edit".to_string()],
                search_hint: "Make exact string replacements in files.".to_string(),
                input_schema: ToolInputSchema {
                    schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "file_path": {
                                "type": "string",
                                "description": "Absolute path to the file to edit"
                            },
                            "old_string": {
                                "type": "string",
                                "description": "The exact string to find and replace"
                            },
                            "new_string": {
                                "type": "string",
                                "description": "The replacement string"
                            },
                            "replace_all": {
                                "type": "boolean",
                                "description": "Replace all occurrences (default: false)"
                            }
                        },
                        "required": ["file_path", "old_string", "new_string"]
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
impl Tool for FileEditTool {
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

        let old_string = input["old_string"]
            .as_str()
            .ok_or_else(|| DomainError::Validation("old_string is required".to_string()))?;

        let new_string = input["new_string"]
            .as_str()
            .ok_or_else(|| DomainError::Validation("new_string is required".to_string()))?;

        let replace_all = input["replace_all"].as_bool().unwrap_or(false);

        // Read current content
        let content = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| DomainError::NotFound(format!("{file_path}: {e}")))?;

        // Check that old_string exists
        let count = content.matches(old_string).count();
        if count == 0 {
            return Err(DomainError::Validation(format!(
                "old_string not found in {file_path}"
            )));
        }

        if !replace_all && count > 1 {
            return Err(DomainError::Validation(format!(
                "old_string found {count} times in {file_path}. Use replace_all or provide more context."
            )));
        }

        // Perform replacement
        let new_content = if replace_all {
            content.replace(old_string, new_string)
        } else {
            content.replacen(old_string, new_string, 1)
        };

        tokio::fs::write(file_path, &new_content)
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to write: {e}")))?;

        let replaced = if replace_all {
            format!("{count} occurrences")
        } else {
            "1 occurrence".to_string()
        };

        Ok(ToolOutput {
            content: format!("Replaced {replaced} in {file_path}"),
            has_new_messages: false,
            has_modifier: false,
        })
    }
}
