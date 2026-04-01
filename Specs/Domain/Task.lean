/-
  Domain/Task.lean — Task 실행 시스템
  백그라운드 태스크, 에이전트, 원격 실행의 상태 머신.
-/
import Common.Types

namespace ClaudeCode.Task

open ClaudeCode

-- ═══════════════════════════════════════════════
-- Task Type & Status (State Machine)
-- ═══════════════════════════════════════════════

/-- 태스크 유형. -/
inductive TaskType where
  | localBash          -- 로컬 쉘 명령
  | localAgent         -- 로컬 서브에이전트
  | remoteAgent        -- 원격 SSH 에이전트
  | inProcessTeammate  -- 인프로세스 팀원
  | localWorkflow      -- 로컬 워크플로우
  | monitorMcp         -- MCP 모니터
  | dream              -- 드림 태스크
  deriving Repr, BEq

/-- 태스크 상태. -/
inductive TaskStatus where
  | pending
  | running
  | completed
  | failed
  | killed
  deriving Repr, BEq

/-- 종료 상태 판별. -/
def TaskStatus.isTerminal : TaskStatus → Bool
  | .completed => true
  | .failed    => true
  | .killed    => true
  | _          => false

-- ═══════════════════════════════════════════════
-- State Machine Transitions
-- ═══════════════════════════════════════════════

/-- 유효한 상태 전이. -/
inductive ValidTransition : TaskStatus → TaskStatus → Prop where
  | pendingToRunning   : ValidTransition .pending .running
  | runningToCompleted : ValidTransition .running .completed
  | runningToFailed    : ValidTransition .running .failed
  | runningToKilled    : ValidTransition .running .killed
  | pendingToKilled    : ValidTransition .pending .killed

/-- 종료 상태에서는 전이 불가. -/
theorem no_transition_from_terminal (s1 s2 : TaskStatus) :
    s1.isTerminal = true → ¬ ValidTransition s1 s2 := by
  intro h_term h_trans
  cases h_trans <;> simp [TaskStatus.isTerminal] at h_term

-- ═══════════════════════════════════════════════
-- Task State
-- ═══════════════════════════════════════════════

/-- 태스크 ID 접두사 규칙. -/
def taskIdPrefix : TaskType → String
  | .localBash         => "sh"
  | .localAgent        => "ag"
  | .remoteAgent       => "ra"
  | .inProcessTeammate => "tm"
  | .localWorkflow     => "wf"
  | .monitorMcp        => "mc"
  | .dream             => "dr"

/-- 태스크 기본 상태. -/
structure TaskState where
  id           : String
  type         : TaskType
  status       : TaskStatus
  description  : String
  toolUseId    : Option String     := none
  startTime    : Timestamp
  endTime      : Option Timestamp  := none
  totalPausedMs : Nat              := 0
  outputFile   : FilePath
  outputOffset : Nat               := 0
  notified     : Bool              := false
  deriving Repr

/-- 로컬 쉘 태스크 입력. -/
structure LocalShellSpawnInput where
  command     : String
  description : String
  timeout     : Option Duration  := none
  toolUseId   : Option String    := none
  agentId     : Option AgentId   := none
  kind        : Option String    := none  -- "bash" | "monitor"
  deriving Repr

-- ═══════════════════════════════════════════════
-- Invariants
-- ═══════════════════════════════════════════════

/-- 완료된 태스크는 종료 시간이 있다. -/
structure CompletedTaskInvariant (t : TaskState) : Prop where
  statusIsTerminal : t.status.isTerminal = true
  hasEndTime       : t.endTime.isSome = true

/-- 실행 중 태스크는 종료 시간이 없다. -/
theorem running_has_no_end_time (t : TaskState) :
    t.status = .running → t.endTime = none → t.endTime.isSome = false := by
  intro _ h_none
  simp [h_none]

-- ═══════════════════════════════════════════════
-- BoundaryMap
-- ═══════════════════════════════════════════════

def taskBoundaryMap : List BoundaryEntry := [
  { fnName := "TaskState",      traits := [.pure, .provable], target := .typescript },
  { fnName := "TaskType",       traits := [.pure],            target := .typescript },
  { fnName := "TaskStatus",     traits := [.pure, .provable], target := .typescript },
  { fnName := "ValidTransition", traits := [.pure, .provable], target := .typescript }
]

end ClaudeCode.Task
