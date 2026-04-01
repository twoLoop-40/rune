// @spec Specs/Domain/Query.lean ClaudeCode.Query
//
// Claude Code Engine — the execution loop.
//
// Architecture:
//   LlmProvider::stream(request)
//     → StreamEvent (content_block_start/delta/stop, message_stop)
//     → on tool_use: ToolExecutor dispatches to Tool::execute()
//     → collect tool results → append to messages → loop
//     → on end_turn/stop: return final result
//
// This is the heart of the engine. LLM-agnostic, tool-agnostic.

pub mod executor;
pub mod query_engine;
