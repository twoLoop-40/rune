// rune — LLM-agnostic agent engine CLI
//
// Usage:
//   rune                           # interactive REPL
//   rune "explain this project"    # single-shot mode
//   rune --model claude-opus-4-20250514 "fix the bug"

mod session;

use std::io::Write;
use std::sync::Arc;

use clap::Parser;

use claude_code_api::claude::ClaudeProvider;
use claude_code_core::llm::LlmProvider;
use claude_code_core::message::{MessageRole, MessageType, SerializedMessage};
use claude_code_core::permission::PermissionDecision;
use claude_code_core::query::{QueryEngineConfig, ThinkingConfig, TokenUsage};
use claude_code_core::types::{ContentBlock, FilePath, SessionId, Timestamp, Uuid};
use claude_code_engine::executor::ToolExecutor;
use claude_code_engine::query_engine::{QueryEngine, StreamHandler};
use claude_code_tools::builtin::bash::BashTool;
use claude_code_tools::builtin::file_edit::FileEditTool;
use claude_code_tools::builtin::file_read::FileReadTool;
use claude_code_tools::builtin::file_write::FileWriteTool;
use claude_code_tools::builtin::glob::GlobTool;
use claude_code_tools::builtin::grep::GrepTool;
use claude_code_tools::{HookRunner, PermissionChecker, ToolContext, ToolOutput, ToolRegistry};

// ═══════════════════════════════════════════════
// CLI args
// ═══════════════════════════════════════════════

#[derive(Parser)]
#[command(name = "rune", about = "LLM-agnostic agent engine")]
struct Cli {
    /// The prompt to send (omit for interactive REPL)
    prompt: Option<String>,

    /// Model to use
    #[arg(long, default_value = "claude-sonnet-4-20250514")]
    model: String,

    /// Max turns per query
    #[arg(long, default_value = "10")]
    max_turns: u32,

    /// Resume a previous session by ID
    #[arg(long)]
    resume: Option<String>,
}

// ═══════════════════════════════════════════════
// Trait implementations
// ═══════════════════════════════════════════════

struct AllowAll;

#[async_trait::async_trait]
impl PermissionChecker for AllowAll {
    async fn check(
        &self,
        _tool_name: &str,
        _input: &serde_json::Value,
        _ctx: &ToolContext,
    ) -> PermissionDecision {
        PermissionDecision::Allow {
            user_modified: false,
        }
    }
}

struct NoHooks;

#[async_trait::async_trait]
impl HookRunner for NoHooks {
    async fn pre_tool_use(
        &self,
        _tool_name: &str,
        _input: &serde_json::Value,
    ) -> Option<claude_code_core::hook::HookDecision> {
        None
    }

    async fn post_tool_use(
        &self,
        _tool_name: &str,
        _input: &serde_json::Value,
        _output: &ToolOutput,
    ) {
    }
}

struct StdoutHandler;

#[async_trait::async_trait]
impl StreamHandler for StdoutHandler {
    async fn on_text(&self, text: &str) {
        print!("{text}");
        std::io::stdout().flush().ok();
    }

    async fn on_tool_use_start(&self, tool_name: &str, _tool_use_id: &str) {
        eprintln!("\n⚡ {tool_name}");
    }

    async fn on_tool_result(&self, _tool_use_id: &str, result: &str) {
        let preview = if result.len() > 200 {
            format!("{}...", &result[..200])
        } else {
            result.to_string()
        };
        eprintln!("  → {preview}");
    }

    async fn on_thinking(&self, _text: &str) {}

    async fn on_turn_complete(&self, usage: &TokenUsage) {
        if usage.total() > 0 {
            eprintln!(
                "\n[tokens: {}in + {}out = {}]",
                usage.input_tokens,
                usage.output_tokens,
                usage.total()
            );
        }
    }
}

// ═══════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════

fn make_user_message(text: String, cwd: &str) -> SerializedMessage {
    SerializedMessage {
        uuid: Uuid(format!(
            "{:032x}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        )),
        message_type: MessageType::User,
        role: MessageRole::User,
        content: vec![ContentBlock::Text { content: text }],
        cwd: FilePath(cwd.to_string()),
        session_id: SessionId("cli".to_string()),
        timestamp: Timestamp {
            ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        },
        version: 1,
    }
}

fn build_system_prompt(cwd: &str) -> String {
    format!(
        "You are rune, an AI coding assistant running in the terminal.\n\
         \n\
         Current working directory: {cwd}\n\
         \n\
         Available tools:\n\
         - Bash: Execute shell commands\n\
         - FileRead: Read files with line numbers\n\
         - FileEdit: Make exact string replacements in files\n\
         - FileWrite: Create or overwrite files\n\
         - Grep: Search file contents using regex (ripgrep)\n\
         - Glob: Find files matching patterns\n\
         \n\
         Guidelines:\n\
         - Always read files before modifying them\n\
         - Use Grep/Glob to explore the codebase before making changes\n\
         - Be concise in responses\n\
         - Show relevant code when explaining"
    )
}

fn read_line(prompt: &str) -> Option<String> {
    eprint!("{prompt}");
    std::io::stderr().flush().ok();
    let mut buf = String::new();
    match std::io::stdin().read_line(&mut buf) {
        Ok(0) => None, // EOF
        Ok(_) => Some(buf.trim_end().to_string()),
        Err(_) => None,
    }
}

/// Handle slash commands. Returns true if handled (don't send to LLM).
fn handle_slash_command(input: &str, session_tokens: &mut u64) -> bool {
    match input {
        "/help" => {
            eprintln!("  /help      — show this help");
            eprintln!("  /clear     — clear conversation history");
            eprintln!("  /model     — show current model");
            eprintln!("  /tokens    — show token usage");
            eprintln!("  /save      — save session");
            eprintln!("  /sessions  — list saved sessions");
            eprintln!("  /quit      — exit rune");
            true
        }
        "/clear" => {
            eprintln!("  ✓ Conversation cleared");
            true
        }
        "/tokens" => {
            eprintln!("  Session tokens: {session_tokens}");
            true
        }
        "/model" => {
            // Model info is printed in main loop
            true
        }
        _ if input.starts_with('/') => {
            eprintln!("  Unknown command: {input}. Type /help for available commands.");
            true
        }
        _ => false,
    }
}

// ═══════════════════════════════════════════════
// Main
// ═══════════════════════════════════════════════

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY not set. Add it to .env or export it."))?;

    let cwd = std::env::current_dir()?
        .to_string_lossy()
        .to_string();

    // Build components
    let provider = Arc::new(ClaudeProvider::new(api_key)) as Arc<dyn LlmProvider>;

    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(BashTool::new()));
    registry.register(Arc::new(FileEditTool::new()));
    registry.register(Arc::new(FileReadTool::new()));
    registry.register(Arc::new(FileWriteTool::new()));
    registry.register(Arc::new(GlobTool::new()));
    registry.register(Arc::new(GrepTool::new()));
    let registry = Arc::new(registry);

    let executor = Arc::new(ToolExecutor::new(
        Arc::clone(&registry),
        Arc::new(AllowAll),
        Arc::new(NoHooks),
    ));

    let config = QueryEngineConfig {
        cwd: FilePath(cwd.clone()),
        max_turns: Some(cli.max_turns),
        max_budget_usd: None,
        verbose: false,
        user_specified_model: Some(cli.model.clone()),
        fallback_model: None,
        thinking_config: ThinkingConfig::default(),
        has_json_schema: false,
    };

    let engine = QueryEngine::new(
        provider,
        executor,
        Arc::clone(&registry),
        config,
        Arc::new(StdoutHandler),
    );

    let system_prompt = build_system_prompt(&cwd);

    // Single-shot mode
    if let Some(prompt) = cli.prompt {
        let msg = make_user_message(prompt, &cwd);
        let result = engine.run(&system_prompt, vec![msg]).await;
        match result {
            Ok(r) => {
                eprintln!(
                    "\n✓ Done ({} turns, {} tokens)",
                    r.turn_count,
                    r.total_usage.total()
                );
            }
            Err(e) => {
                eprintln!("\n✗ Error: {e}");
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    // ═══════════════════════════════════════════════
    // Interactive REPL mode
    // ═══════════════════════════════════════════════

    eprintln!("rune v0.1.0 — LLM-agnostic agent engine");
    eprintln!("model: {}", cli.model);
    eprintln!("cwd: {cwd}");
    eprintln!("tools: {} loaded", registry.len());
    eprintln!("Type /help for commands, /quit to exit.\n");

    // Session management
    let session_id = cli.resume.clone().unwrap_or_else(|| session::new_session_id());
    let mut conversation: Vec<SerializedMessage> = if cli.resume.is_some() {
        match session::load_session(&session_id) {
            Ok(msgs) => {
                eprintln!("  Resumed session: {session_id} ({} messages)", msgs.len());
                msgs
            }
            Err(e) => {
                eprintln!("  Failed to load session: {e}");
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };
    let mut session_tokens: u64 = 0;

    eprintln!("session: {session_id}\n");

    loop {
        let input = match read_line("rune> ") {
            Some(s) if s.is_empty() => continue,
            Some(s) => s,
            None => break, // EOF (Ctrl+D)
        };

        // Quit
        if input == "/quit" || input == "/exit" {
            if !conversation.is_empty() {
                if let Err(e) = session::save_session(&session_id, &conversation) {
                    eprintln!("  Warning: failed to save session: {e}");
                } else {
                    eprintln!("  Session saved: {session_id}");
                }
            }
            eprintln!("Bye! ({} tokens)", session_tokens);
            break;
        }

        // Clear conversation
        if input == "/clear" {
            conversation.clear();
            session_tokens = 0;
            handle_slash_command("/clear", &mut session_tokens);
            continue;
        }

        // Model info
        if input == "/model" {
            eprintln!("  Model: {}", cli.model);
            continue;
        }

        // List sessions
        if input == "/sessions" {
            match session::list_sessions() {
                Ok(sessions) if sessions.is_empty() => {
                    eprintln!("  No saved sessions");
                }
                Ok(sessions) => {
                    eprintln!("  Recent sessions:");
                    for (id, _) in sessions.iter().take(10) {
                        eprintln!("    {id}");
                    }
                    eprintln!("  Resume with: rune --resume <id>");
                }
                Err(e) => eprintln!("  Error: {e}"),
            }
            continue;
        }

        // Save current session
        if input == "/save" {
            match session::save_session(&session_id, &conversation) {
                Ok(()) => eprintln!("  Session saved: {session_id}"),
                Err(e) => eprintln!("  Error: {e}"),
            }
            continue;
        }

        // Other slash commands
        if handle_slash_command(&input, &mut session_tokens) {
            continue;
        }

        // Build user message and run
        let msg = make_user_message(input, &cwd);
        conversation.push(msg);

        match engine.run(&system_prompt, conversation.clone()).await {
            Ok(result) => {
                let turn_tokens = result.total_usage.total();
                session_tokens += turn_tokens;

                // Keep the full conversation (including assistant messages)
                conversation = result.messages;

                eprintln!();
            }
            Err(e) => {
                eprintln!("\n✗ Error: {e}");
                // Remove the failed user message
                conversation.pop();
            }
        }
    }

    Ok(())
}
