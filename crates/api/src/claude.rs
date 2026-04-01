// ClaudeProvider — Anthropic Messages API with SSE streaming.
//
// Lean: QueryEngineConfig, StreamEvent, FinishReason, TokenUsage
// Implements: LlmProvider trait from core::llm

use std::pin::Pin;

use reqwest_eventsource::{Event, EventSource};
use tokio_stream::Stream;

use claude_code_core::llm::{LlmProvider, LlmRequest, LlmStream};
use claude_code_core::query::{FinishReason, StreamEvent};
use claude_code_core::types::DomainError;

/// Anthropic Claude API provider.
pub struct ClaudeProvider {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl ClaudeProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.anthropic.com".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    /// Build the API request body.
    fn build_body(&self, request: &LlmRequest) -> serde_json::Value {
        let mut messages = Vec::new();
        for msg in &request.messages {
            let content: Vec<serde_json::Value> = msg
                .content
                .iter()
                .map(|block| match block {
                    claude_code_core::types::ContentBlock::Text { content } => {
                        serde_json::json!({ "type": "text", "text": content })
                    }
                    claude_code_core::types::ContentBlock::ToolUse { id, name, input } => {
                        serde_json::json!({
                            "type": "tool_use",
                            "id": id,
                            "name": name,
                            "input": input
                        })
                    }
                    claude_code_core::types::ContentBlock::ToolResult {
                        tool_use_id,
                        content,
                        is_error,
                    } => {
                        serde_json::json!({
                            "type": "tool_result",
                            "tool_use_id": tool_use_id,
                            "content": content,
                            "is_error": is_error
                        })
                    }
                    claude_code_core::types::ContentBlock::Thinking { content } => {
                        serde_json::json!({ "type": "thinking", "thinking": content })
                    }
                    claude_code_core::types::ContentBlock::Image { media_type, data } => {
                        serde_json::json!({
                            "type": "image",
                            "source": { "type": "base64", "media_type": media_type, "data": data }
                        })
                    }
                })
                .collect();

            let role = match msg.role {
                claude_code_core::message::MessageRole::User => "user",
                claude_code_core::message::MessageRole::Assistant => "assistant",
                claude_code_core::message::MessageRole::System => continue, // system goes in system param
            };

            messages.push(serde_json::json!({
                "role": role,
                "content": content
            }));
        }

        let mut body = serde_json::json!({
            "model": request.model,
            "max_tokens": request.max_tokens,
            "messages": messages,
            "stream": true
        });

        if !request.system_prompt.is_empty() {
            body["system"] = serde_json::json!(request.system_prompt);
        }

        // Tools
        if !request.tools.is_empty() {
            let tools: Vec<serde_json::Value> = request
                .tools
                .iter()
                .map(|t| {
                    serde_json::json!({
                        "name": t.name,
                        "description": t.description,
                        "input_schema": t.input_schema
                    })
                })
                .collect();
            body["tools"] = serde_json::json!(tools);
        }

        body
    }
}

#[async_trait::async_trait]
impl LlmProvider for ClaudeProvider {
    async fn stream(&self, request: LlmRequest) -> Result<LlmStream, DomainError> {
        let body = self.build_body(&request);
        let url = format!("{}/v1/messages", self.base_url);

        let req = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .body(body.to_string());

        let es = EventSource::new(req)
            .map_err(|e| DomainError::Internal(format!("SSE connection failed: {e}")))?;

        let stream = SseStream { es, done: false };

        Ok(Box::pin(stream))
    }

    fn provider_name(&self) -> &str {
        "anthropic"
    }

    fn supports_model(&self, model: &str) -> bool {
        model.starts_with("claude-")
    }
}

/// Wraps reqwest-eventsource into a tokio Stream of StreamEvents.
struct SseStream {
    es: EventSource,
    done: bool,
}

impl Stream for SseStream {
    type Item = Result<StreamEvent, DomainError>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.done {
            return std::task::Poll::Ready(None);
        }

        match Pin::new(&mut self.es).poll_next(cx) {
            std::task::Poll::Ready(Some(Ok(event))) => match event {
                Event::Open => {
                    cx.waker().wake_by_ref();
                    std::task::Poll::Pending
                }
                Event::Message(msg) => {
                    let parsed = parse_sse_event(&msg.event, &msg.data);
                    match parsed {
                        Some(Ok(event)) => {
                            if matches!(event, StreamEvent::MessageStop { .. }) {
                                self.done = true;
                            }
                            std::task::Poll::Ready(Some(Ok(event)))
                        }
                        Some(Err(e)) => std::task::Poll::Ready(Some(Err(e))),
                        None => {
                            // Unknown event type, skip
                            cx.waker().wake_by_ref();
                            std::task::Poll::Pending
                        }
                    }
                }
            },
            std::task::Poll::Ready(Some(Err(e))) => {
                self.done = true;
                std::task::Poll::Ready(Some(Err(DomainError::Internal(format!(
                    "SSE error: {e}"
                )))))
            }
            std::task::Poll::Ready(None) => {
                self.done = true;
                std::task::Poll::Ready(None)
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Parse Anthropic SSE event into our StreamEvent.
fn parse_sse_event(event_type: &str, data: &str) -> Option<Result<StreamEvent, DomainError>> {
    let json: serde_json::Value = match serde_json::from_str(data) {
        Ok(v) => v,
        Err(e) => return Some(Err(DomainError::Internal(format!("JSON parse error: {e}")))),
    };

    match event_type {
        "message_start" => Some(Ok(StreamEvent::MessageStart)),

        "content_block_start" => {
            let index = json["index"].as_u64().unwrap_or(0) as u32;
            let block_type = json["content_block"]["type"]
                .as_str()
                .unwrap_or("text")
                .to_string();
            let tool_use_id = json["content_block"]["id"]
                .as_str()
                .map(|s| s.to_string());
            let tool_name = json["content_block"]["name"]
                .as_str()
                .map(|s| s.to_string());
            Some(Ok(StreamEvent::ContentBlockStart {
                index,
                block_type,
                tool_use_id,
                tool_name,
            }))
        }

        "content_block_delta" => {
            let index = json["index"].as_u64().unwrap_or(0) as u32;
            let delta = if let Some(text) = json["delta"]["text"].as_str() {
                text.to_string()
            } else if let Some(input) = json["delta"]["partial_json"].as_str() {
                input.to_string()
            } else {
                String::new()
            };
            Some(Ok(StreamEvent::ContentBlockDelta { index, delta }))
        }

        "content_block_stop" => {
            let index = json["index"].as_u64().unwrap_or(0) as u32;
            Some(Ok(StreamEvent::ContentBlockStop { index }))
        }

        "message_delta" => {
            let stop_reason = json["delta"]["stop_reason"]
                .as_str()
                .unwrap_or("end_turn");
            let finish_reason = match stop_reason {
                "end_turn" => FinishReason::EndTurn,
                "stop_sequence" => FinishReason::Stop,
                "max_tokens" => FinishReason::MaxTokens,
                "tool_use" => FinishReason::ToolUse,
                _ => FinishReason::EndTurn,
            };
            Some(Ok(StreamEvent::MessageStop { finish_reason }))
        }

        "message_stop" => None, // Already handled by message_delta

        "ping" | "error" => None,

        _ => None,
    }
}
