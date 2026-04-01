// StreamingToolExecutor — parallel tool dispatch.
//
// When the LLM emits multiple tool_use blocks in a single response,
// this executor runs them concurrently (if safe) and collects results.

use std::sync::Arc;

use claude_code_core::hook::HookDecision;
use claude_code_core::permission::PermissionDecision;
use claude_code_tools::{HookRunner, PermissionChecker, Tool, ToolContext, ToolOutput, ToolRegistry};

/// A pending tool call extracted from LLM output.
#[derive(Debug, Clone)]
pub struct ToolCall {
    pub tool_use_id: String,
    pub tool_name: String,
    pub input: serde_json::Value,
}

/// Result of a single tool execution.
#[derive(Debug, Clone)]
pub struct ToolCallResult {
    pub tool_use_id: String,
    pub tool_name: String,
    pub output: Result<ToolOutput, String>,
}

/// Executes tool calls, respecting permissions and hooks.
pub struct ToolExecutor {
    pub registry: Arc<ToolRegistry>,
    pub permission_checker: Arc<dyn PermissionChecker>,
    pub hook_runner: Arc<dyn HookRunner>,
}

impl ToolExecutor {
    pub fn new(
        registry: Arc<ToolRegistry>,
        permission_checker: Arc<dyn PermissionChecker>,
        hook_runner: Arc<dyn HookRunner>,
    ) -> Self {
        Self {
            registry,
            permission_checker,
            hook_runner,
        }
    }

    /// Execute a batch of tool calls.
    /// Concurrency-safe tools run in parallel; others run sequentially.
    pub async fn execute_batch(
        &self,
        calls: Vec<ToolCall>,
        ctx: &ToolContext,
    ) -> Vec<ToolCallResult> {
        let (concurrent, sequential): (Vec<ToolCall>, Vec<ToolCall>) =
            calls.into_iter().partition(|call: &ToolCall| {
                self.registry
                    .get(&call.tool_name)
                    .is_some_and(|t| t.capabilities().is_concurrency_safe)
            });

        let mut results = Vec::new();

        // Run concurrent tools in parallel
        if !concurrent.is_empty() {
            let handles: Vec<_> = concurrent
                .into_iter()
                .map(|call| {
                    let registry = Arc::clone(&self.registry);
                    let perm = Arc::clone(&self.permission_checker);
                    let hooks = Arc::clone(&self.hook_runner);
                    let ctx = ctx.clone();
                    tokio::spawn(async move {
                        execute_single_call(&registry, &*perm, &*hooks, call, &ctx).await
                    })
                })
                .collect();

            for handle in handles {
                match handle.await {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        tracing::error!("Tool task panicked: {e}");
                    }
                }
            }
        }

        // Run sequential tools one by one
        for call in sequential {
            let result =
                execute_single_call(&self.registry, &*self.permission_checker, &*self.hook_runner, call, ctx)
                    .await;
            results.push(result);
        }

        results
    }
}

/// Execute a single tool call with permission check and hooks.
async fn execute_single_call(
    registry: &ToolRegistry,
    perm: &dyn PermissionChecker,
    hooks: &dyn HookRunner,
    call: ToolCall,
    ctx: &ToolContext,
) -> ToolCallResult {
    let tool_use_id = call.tool_use_id.clone();
    let tool_name = call.tool_name.clone();

    let result = execute_inner(registry, perm, hooks, call, ctx).await;

    ToolCallResult {
        tool_use_id,
        tool_name,
        output: result,
    }
}

async fn execute_inner(
    registry: &ToolRegistry,
    perm: &dyn PermissionChecker,
    hooks: &dyn HookRunner,
    call: ToolCall,
    ctx: &ToolContext,
) -> Result<ToolOutput, String> {
    // 1. Find tool
    let tool = registry
        .get(&call.tool_name)
        .ok_or_else(|| format!("Unknown tool: {}", call.tool_name))?;

    // 2. Validate input
    tool.validate_input(&call.input)
        .map_err(|e| format!("Validation error: {e}"))?;

    // 3. Check permissions (Lean: Permission.lean flow)
    let decision = perm.check(&call.tool_name, &call.input, ctx).await;
    match &decision {
        PermissionDecision::Deny { message, .. } => {
            return Err(format!("Permission denied: {message}"));
        }
        PermissionDecision::Ask { message, .. } => {
            // In a real implementation, this would prompt the user
            // For now, treat as denied
            return Err(format!("Permission required: {message}"));
        }
        PermissionDecision::Passthrough { .. } => {
            // Delegate to hook
        }
        PermissionDecision::Allow { .. } => {
            // Proceed
        }
    }

    // 4. Pre-tool-use hook (Lean: HookEvent.preToolUse)
    if let Some(HookDecision::Block { reason }) =
        hooks.pre_tool_use(&call.tool_name, &call.input).await
    {
        return Err(format!("Blocked by hook: {reason}"));
    }

    // 5. Execute tool
    let tool_ctx = ToolContext {
        cwd: ctx.cwd.clone(),
        agent_id: ctx.agent_id.clone(),
        tool_use_id: call.tool_use_id.clone(),
    };

    let output = Tool::execute(&**tool, call.input, &tool_ctx)
        .await
        .map_err(|e| format!("Tool execution error: {e}"))?;

    // 6. Post-tool-use hook
    hooks
        .post_tool_use(&call.tool_name, &serde_json::Value::Null, &output)
        .await;

    Ok(output)
}
