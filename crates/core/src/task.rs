// @spec Specs/Domain/Task.lean ClaudeCode.Task
//
// Task execution system — background tasks, agents, remote execution.
// Lean ValidTransition prop → Rust typestate pattern.

use serde::{Deserialize, Serialize};

use crate::types::{AgentId, Duration, FilePath, Timestamp};

// ═══════════════════════════════════════════════
// Task Type
// ═══════════════════════════════════════════════

/// Lean: `inductive TaskType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskType {
    LocalBash,
    LocalAgent,
    RemoteAgent,
    InProcessTeammate,
    LocalWorkflow,
    MonitorMcp,
    Dream,
}

impl TaskType {
    /// Lean: `def taskIdPrefix`
    pub const fn id_prefix(self) -> &'static str {
        match self {
            Self::LocalBash => "sh",
            Self::LocalAgent => "ag",
            Self::RemoteAgent => "ra",
            Self::InProcessTeammate => "tm",
            Self::LocalWorkflow => "wf",
            Self::MonitorMcp => "mc",
            Self::Dream => "dr",
        }
    }
}

// ═══════════════════════════════════════════════
// Task Status (runtime enum for serialization)
// ═══════════════════════════════════════════════

/// Lean: `inductive TaskStatus`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Killed,
}

impl TaskStatus {
    /// Lean: `def TaskStatus.isTerminal`
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Killed)
    }
}

// ═══════════════════════════════════════════════
// Typestate — compile-time transition enforcement
// Lean: `inductive ValidTransition` (Prop)
//
// Valid transitions:
//   Pending  → Running
//   Running  → Completed | Failed | Killed
//   Pending  → Killed
//
// Lean theorem: `no_transition_from_terminal`
// → In Rust: terminal states have no transition methods.
// ═══════════════════════════════════════════════

/// Marker traits for task states (typestate pattern).
pub mod states {
    pub struct Pending;
    pub struct Running;
    pub struct Completed;
    pub struct Failed;
    pub struct Killed;
}

/// Typed task — state is encoded in the type parameter.
/// Terminal states (Completed, Failed, Killed) have no transition methods,
/// enforcing `no_transition_from_terminal` at compile time.
#[derive(Debug)]
pub struct Task<S> {
    pub id: String,
    pub task_type: TaskType,
    pub description: String,
    pub tool_use_id: Option<String>,
    pub start_time: Timestamp,
    pub end_time: Option<Timestamp>,
    pub total_paused_ms: u64,
    pub output_file: FilePath,
    pub output_offset: u64,
    pub notified: bool,
    _state: std::marker::PhantomData<S>,
}

impl<S> Task<S> {
    fn transition<T>(self) -> Task<T> {
        Task {
            id: self.id,
            task_type: self.task_type,
            description: self.description,
            tool_use_id: self.tool_use_id,
            start_time: self.start_time,
            end_time: self.end_time,
            total_paused_ms: self.total_paused_ms,
            output_file: self.output_file,
            output_offset: self.output_offset,
            notified: self.notified,
            _state: std::marker::PhantomData,
        }
    }
}

/// Lean: `ValidTransition .pending .running`
impl Task<states::Pending> {
    pub fn start(self) -> Task<states::Running> {
        self.transition()
    }

    /// Lean: `ValidTransition .pending .killed`
    pub fn kill(self) -> Task<states::Killed> {
        self.transition()
    }
}

/// Lean: `ValidTransition .running .completed/failed/killed`
impl Task<states::Running> {
    pub fn complete(mut self, end_time: Timestamp) -> Task<states::Completed> {
        self.end_time = Some(end_time);
        self.transition()
    }

    pub fn fail(mut self, end_time: Timestamp) -> Task<states::Failed> {
        self.end_time = Some(end_time);
        self.transition()
    }

    pub fn kill(mut self, end_time: Timestamp) -> Task<states::Killed> {
        self.end_time = Some(end_time);
        self.transition()
    }
}

// No transition methods on Completed, Failed, Killed
// → `no_transition_from_terminal` enforced by absence of impl blocks

/// Create a new pending task.
pub fn new_task(
    id: String,
    task_type: TaskType,
    description: String,
    start_time: Timestamp,
    output_file: FilePath,
) -> Task<states::Pending> {
    Task {
        id,
        task_type,
        description,
        tool_use_id: None,
        start_time,
        end_time: None,
        total_paused_ms: 0,
        output_file,
        output_offset: 0,
        notified: false,
        _state: std::marker::PhantomData,
    }
}

// ═══════════════════════════════════════════════
// Runtime TaskState (for serialization / storage)
// ═══════════════════════════════════════════════

/// Runtime task state for serialization.
/// Lean: `structure TaskState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskState {
    pub id: String,
    #[serde(rename = "type")]
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub description: String,
    pub tool_use_id: Option<String>,
    pub start_time: Timestamp,
    pub end_time: Option<Timestamp>,
    pub total_paused_ms: u64,
    pub output_file: FilePath,
    pub output_offset: u64,
    pub notified: bool,
}

// ═══════════════════════════════════════════════
// Local Shell Spawn Input
// ═══════════════════════════════════════════════

/// Lean: `structure LocalShellSpawnInput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalShellSpawnInput {
    pub command: String,
    pub description: String,
    pub timeout: Option<Duration>,
    pub tool_use_id: Option<String>,
    pub agent_id: Option<AgentId>,
    /// "bash" | "monitor"
    pub kind: Option<String>,
}

// ═══════════════════════════════════════════════
// Invariant (runtime assertion)
// Lean: `structure CompletedTaskInvariant`
// ═══════════════════════════════════════════════

impl TaskState {
    /// Lean: `CompletedTaskInvariant` — terminal tasks must have end_time.
    pub fn validate_completed(&self) -> bool {
        if self.status.is_terminal() {
            self.end_time.is_some()
        } else {
            true
        }
    }
}
