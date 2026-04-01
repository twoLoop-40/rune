// QueryEngine — the main execution loop.
//
// Lean: ExecutionPhase (buildPrompt → apiRequest → streamProcessing
//       → toolExecution → continueLoop | terminate)
//
// This is LLM-agnostic. Swap the LlmProvider and everything still works.

use std::sync::Arc;

use tokio_stream::StreamExt;

use claude_code_core::llm::{LlmProvider, LlmRequest, LlmToolDef};
use claude_code_core::message::{MessageRole, MessageType, SerializedMessage};
use claude_code_core::query::{FinishReason, QueryEngineConfig, StreamEvent, TokenUsage};
use claude_code_core::types::{ContentBlock, DomainError, FilePath, SessionId, Timestamp, Uuid};
use claude_code_tools::{ToolContext, ToolRegistry};

use crate::executor::{ToolCall, ToolExecutor};

/// Callback for streaming events (UI rendering, logging, etc.)
#[async_trait::async_trait]
pub trait StreamHandler: Send + Sync {
    /// Called when the LLM starts generating text.
    async fn on_text(&self, text: &str);
    /// Called when the LLM starts a tool call.
    async fn on_tool_use_start(&self, tool_name: &str, tool_use_id: &str);
    /// Called when a tool finishes.
    async fn on_tool_result(&self, tool_use_id: &str, result: &str);
    /// Called when thinking content arrives.
    async fn on_thinking(&self, text: &str);
    /// Called when the turn completes.
    async fn on_turn_complete(&self, usage: &TokenUsage);
}

/// The engine. Takes an LLM provider and tools, runs the agentic loop.
pub struct QueryEngine {
    llm: Arc<dyn LlmProvider>,
    tool_executor: Arc<ToolExecutor>,
    tool_registry: Arc<ToolRegistry>,
    config: QueryEngineConfig,
    handler: Arc<dyn StreamHandler>,
}

/// Result of running the engine.
pub struct EngineResult {
    pub messages: Vec<SerializedMessage>,
    pub total_usage: TokenUsage,
    pub turn_count: u32,
    pub finish_reason: FinishReason,
}

impl QueryEngine {
    pub fn new(
        llm: Arc<dyn LlmProvider>,
        tool_executor: Arc<ToolExecutor>,
        tool_registry: Arc<ToolRegistry>,
        config: QueryEngineConfig,
        handler: Arc<dyn StreamHandler>,
    ) -> Self {
        Self {
            llm,
            tool_executor,
            tool_registry,
            config,
            handler,
        }
    }

    /// Run the engine with the given user prompt.
    /// This is the main entry point.
    ///
    /// Lean: ExecutionPhase loop
    /// ```text
    /// buildPrompt → apiRequest → streamProcessing
    ///   → toolExecution → continueLoop (back to apiRequest)
    ///   → terminate (done)
    /// ```
    pub async fn run(
        &self,
        system_prompt: &str,
        messages: Vec<SerializedMessage>,
    ) -> Result<EngineResult, DomainError> {
        let mut conversation = messages;
        let mut total_usage = TokenUsage::default();
        let mut turn_count = 0u32;
        let max_turns = self.config.max_turns.unwrap_or(u32::MAX);

        loop {
            if turn_count >= max_turns {
                tracing::info!("Max turns ({max_turns}) reached");
                return Ok(EngineResult {
                    messages: conversation,
                    total_usage,
                    turn_count,
                    finish_reason: FinishReason::MaxTokens,
                });
            }

            // Phase: buildPrompt + apiRequest
            let tool_defs = self.build_tool_defs();
            let request = LlmRequest {
                system_prompt: system_prompt.to_string(),
                messages: conversation.clone(),
                model: self
                    .config
                    .user_specified_model
                    .clone()
                    .unwrap_or_else(|| "default".to_string()),
                max_tokens: 16384,
                tools: tool_defs,
                thinking: if self.config.thinking_config.enabled {
                    Some(claude_code_core::llm::ThinkingParams {
                        enabled: true,
                        budget_tokens: self
                            .config
                            .thinking_config
                            .budget_ms
                            .map(|ms| ms as u32),
                    })
                } else {
                    None
                },
            };

            // Phase: streamProcessing
            let mut stream = self.llm.stream(request).await?;

            let mut assistant_content: Vec<ContentBlock> = Vec::new();
            let mut tool_calls: Vec<ToolCall> = Vec::new();
            let mut current_text = String::new();
            let turn_usage = TokenUsage::default();
            let mut finish_reason = FinishReason::EndTurn;

            while let Some(event) = stream.next().await {
                match event? {
                    StreamEvent::ContentBlockStart { index: _, block_type: _ } => {
                        current_text.clear();
                    }
                    StreamEvent::ContentBlockDelta { index: _, delta } => {
                        // Accumulate delta into current block
                        // In a real implementation, we'd distinguish text vs tool_use deltas
                        current_text.push_str(&delta);
                        self.handler.on_text(&delta).await;
                    }
                    StreamEvent::ContentBlockStop { index: _ } => {
                        if !current_text.is_empty() {
                            assistant_content.push(ContentBlock::Text {
                                content: current_text.clone(),
                            });
                        }
                    }
                    StreamEvent::MessageStart => {}
                    StreamEvent::MessageStop { finish_reason: fr } => {
                        finish_reason = fr;
                    }
                }
            }

            // Add assistant message to conversation
            let assistant_msg = make_message(
                MessageRole::Assistant,
                MessageType::Assistant,
                assistant_content.clone(),
                &self.config.cwd,
            );
            conversation.push(assistant_msg);

            self.handler.on_turn_complete(&turn_usage).await;
            total_usage.input_tokens += turn_usage.input_tokens;
            total_usage.output_tokens += turn_usage.output_tokens;
            total_usage.cache_creation += turn_usage.cache_creation;
            total_usage.cache_read += turn_usage.cache_read;
            turn_count += 1;

            // Phase: toolExecution (if any tool_use blocks)
            // Extract tool calls from assistant content
            for block in &assistant_content {
                if let ContentBlock::ToolUse { id, name, input } = block {
                    tool_calls.push(ToolCall {
                        tool_use_id: id.clone(),
                        tool_name: name.clone(),
                        input: input.clone(),
                    });
                }
            }

            if tool_calls.is_empty() {
                // No tool calls → terminal
                return Ok(EngineResult {
                    messages: conversation,
                    total_usage,
                    turn_count,
                    finish_reason,
                });
            }

            // Execute tools
            let ctx = ToolContext {
                cwd: self.config.cwd.0.clone(),
                agent_id: None,
                tool_use_id: String::new(),
            };

            let results = self.tool_executor.execute_batch(tool_calls.clone(), &ctx).await;

            // Build tool result messages
            let mut tool_result_blocks = Vec::new();
            for result in &results {
                let (content, is_error) = match &result.output {
                    Ok(output) => {
                        self.handler
                            .on_tool_result(&result.tool_use_id, &output.content)
                            .await;
                        (output.content.clone(), false)
                    }
                    Err(e) => {
                        self.handler
                            .on_tool_result(&result.tool_use_id, e)
                            .await;
                        (e.clone(), true)
                    }
                };
                tool_result_blocks.push(ContentBlock::ToolResult {
                    tool_use_id: result.tool_use_id.clone(),
                    content,
                    is_error,
                });
            }

            let tool_result_msg = make_message(
                MessageRole::User,
                MessageType::User,
                tool_result_blocks,
                &self.config.cwd,
            );
            conversation.push(tool_result_msg);

            // Clear for next turn
            tool_calls.clear();

            // Phase: continueLoop → back to apiRequest
        }
    }

    fn build_tool_defs(&self) -> Vec<LlmToolDef> {
        self.tool_registry
            .definitions()
            .into_iter()
            .filter(|def| def.capabilities.is_enabled)
            .map(|def| LlmToolDef {
                name: def.name.clone(),
                description: def.search_hint.clone(),
                input_schema: def.input_schema.schema.clone(),
            })
            .collect()
    }
}

/// Helper to construct a SerializedMessage.
fn make_message(
    role: MessageRole,
    msg_type: MessageType,
    content: Vec<ContentBlock>,
    cwd: &FilePath,
) -> SerializedMessage {
    SerializedMessage {
        uuid: Uuid(uuid_v4()),
        message_type: msg_type,
        role,
        content,
        cwd: cwd.clone(),
        session_id: SessionId("engine".to_string()),
        timestamp: Timestamp { ms: now_ms() },
        version: 1,
    }
}

fn uuid_v4() -> String {
    // Minimal UUID v4 without external dependency
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{t:032x}")
}

fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
