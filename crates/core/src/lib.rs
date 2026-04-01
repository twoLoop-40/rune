// @spec Specs/Common/Types.lean ClaudeCode
// @spec Specs/Domain/Task.lean ClaudeCode.Task
// @spec Specs/Domain/Permission.lean ClaudeCode.Permission
// @spec Specs/Domain/Message.lean ClaudeCode.Message
// @spec Specs/Domain/Query.lean ClaudeCode.Query
// @spec Specs/Domain/State.lean ClaudeCode.State
// @spec Specs/Domain/Hook.lean ClaudeCode.Hook
// @spec Specs/Domain/Tool.lean ClaudeCode.Tool
// @spec Specs/Domain/Plugin.lean ClaudeCode.Plugin
// @spec Specs/Domain/Agent.lean ClaudeCode.Agent
// @spec Specs/Domain/Session.lean ClaudeCode.Session
// @spec Specs/Domain/Model.lean ClaudeCode.Model
// @spec Specs/Domain/Cost.lean ClaudeCode.Cost

pub mod types;
pub mod llm;
pub mod task;
pub mod permission;
pub mod message;
pub mod query;
pub mod state;
pub mod hook;
pub mod tool;
pub mod plugin;
pub mod command;
pub mod bridge;
pub mod agent;
pub mod session;
pub mod model;
pub mod vim;
pub mod worktree;
pub mod file_history;
pub mod voice;
pub mod text_input;
pub mod mcp_server;
pub mod compaction;
pub mod remote_session;
pub mod cost;
