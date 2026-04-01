// rune — LLM-agnostic agent engine CLI
//
// Usage:
//   ANTHROPIC_API_KEY=sk-... rune "explain this project"
//   echo "list files" | rune
//   rune --model claude-sonnet-4-20250514 "what does main.rs do?"

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
use claude_code_tools::builtin::file_read::FileReadTool;
use claude_code_tools::builtin::file_write::FileWriteTool;
use claude_code_tools::{HookRunner, PermissionChecker, ToolContext, ToolOutput, ToolRegistry};

// ═══════════════════════════════════════════════
// CLI args
// ═══════════════════════════════════════════════

#[derive(Parser)]
#[command(name = "rune", about = "LLM-agnostic agent engine")]
struct Cli {
    /// The prompt to send to the LLM
    prompt: Option<String>,

    /// Model to use
    #[arg(long, default_value = "claude-sonnet-4-20250514")]
    model: String,

    /// Max turns before stopping
    #[arg(long, default_value = "10")]
    max_turns: u32,
}

// ═══════════════════════════════════════════════
// Minimal implementations for first run
// ═══════════════════════════════════════════════

/// Allow everything (bypass mode for now).
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

/// No hooks for now.
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

/// Print to stdout as events arrive.
struct StdoutHandler;

#[async_trait::async_trait]
impl StreamHandler for StdoutHandler {
    async fn on_text(&self, text: &str) {
        use std::io::Write;
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
        eprintln!(
            "\n[tokens: {}in + {}out = {}]",
            usage.input_tokens,
            usage.output_tokens,
            usage.total()
        );
    }
}

// ═══════════════════════════════════════════════
// Main
// ═══════════════════════════════════════════════

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Get prompt
    let prompt = match cli.prompt {
        Some(p) => p,
        None => {
            // Read from stdin
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    if prompt.trim().is_empty() {
        eprintln!("Usage: rune \"your prompt here\"");
        std::process::exit(1);
    }

    // API key
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY environment variable not set"))?;

    // CWD
    let cwd = std::env::current_dir()?
        .to_string_lossy()
        .to_string();

    // Build components
    let provider = Arc::new(ClaudeProvider::new(api_key)) as Arc<dyn LlmProvider>;

    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(BashTool::new()));
    registry.register(Arc::new(FileReadTool::new()));
    registry.register(Arc::new(FileWriteTool::new()));
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
        user_specified_model: Some(cli.model),
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

    // Build initial message
    let user_msg = SerializedMessage {
        uuid: Uuid(format!("{:032x}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos())),
        message_type: MessageType::User,
        role: MessageRole::User,
        content: vec![ContentBlock::Text {
            content: prompt,
        }],
        cwd: FilePath(cwd),
        session_id: SessionId("cli".to_string()),
        timestamp: Timestamp {
            ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        },
        version: 1,
    };

    let system_prompt = "You are a helpful AI assistant with access to tools. \
        Use FileRead to read files, Bash to run commands, and FileWrite to create files. \
        Always read files before modifying them.";

    // Run the engine
    let result = engine.run(system_prompt, vec![user_msg]).await;

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

    Ok(())
}
