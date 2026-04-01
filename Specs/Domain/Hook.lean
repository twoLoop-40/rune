/-
  Domain/Hook.lean — 훅 시스템
  이벤트 기반 확장 포인트. 17종 이벤트.
-/
import Common.Types

namespace ClaudeCode.Hook

open ClaudeCode

-- ═══════════════════════════════════════════════
-- Hook Events
-- ═══════════════════════════════════════════════

/-- 훅 이벤트 유형. 17종. -/
inductive HookEvent where
  | sessionStart
  | setup
  | subagentStart
  | preToolUse
  | postToolUse
  | postToolUseFailure
  | userPromptSubmit
  | fileChanged
  | cwdChanged
  | permissionRequest
  | permissionDenied
  | elicitation
  | elicitationResult
  | notification
  | worktreeCreate
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Hook Callback
-- ═══════════════════════════════════════════════

/-- 훅 콜백 정의. -/
structure HookCallback where
  timeout   : Option Nat := none   -- seconds
  internal  : Bool := false
  deriving Repr

-- ═══════════════════════════════════════════════
-- Hook Output
-- ═══════════════════════════════════════════════

/-- 훅 실행 결과 결정. -/
inductive HookDecision where
  | approve
  | block (reason : String)
  deriving Repr, BEq

/-- 훅 JSON 출력. -/
structure HookOutput where
  continue_       : Bool := true
  suppressOutput  : Bool := false
  stopReason      : Option String := none
  decision        : Option HookDecision := none
  systemMessage   : Option String := none
  deriving Repr

/-- 비동기 훅 출력. -/
structure AsyncHookOutput where
  isAsync      : Bool := true
  asyncTimeout : Option Nat := none
  deriving Repr

-- ═══════════════════════════════════════════════
-- Hook Result
-- ═══════════════════════════════════════════════

/-- 훅 실행 결과. -/
inductive HookOutcome where
  | success
  | blocking (error : String)
  | nonBlockingError (error : String)
  | cancelled
  deriving Repr, BEq

/-- 훅 결과. -/
structure HookResult where
  outcome              : HookOutcome
  preventContinuation  : Bool := false
  stopReason           : Option String := none
  permissionBehavior   : Option String := none  -- "ask" | "deny" | "allow" | "passthrough"
  additionalContext    : Option String := none
  deriving Repr

-- ═══════════════════════════════════════════════
-- Hook Registration
-- ═══════════════════════════════════════════════

/-- 훅 등록 엔트리. -/
structure HookRegistration where
  event     : HookEvent
  matcher   : Option String := none   -- 도구 이름 패턴
  callback  : HookCallback
  priority  : Nat := 100              -- 낮을수록 먼저
  deriving Repr

-- ═══════════════════════════════════════════════
-- Invariants
-- ═══════════════════════════════════════════════

/-- PreToolUse 훅이 block하면 도구 실행 불가. -/
structure PreToolUseBlockInvariant : Prop where
  blockPreventsExecution :
    ∀ (d : HookDecision) (reason : String), d = .block reason → d ≠ .approve

theorem block_is_not_approve : ∀ (reason : String),
    HookDecision.block reason ≠ HookDecision.approve := by
  intro _
  simp [HookDecision.block]

-- ═══════════════════════════════════════════════
-- BoundaryMap
-- ═══════════════════════════════════════════════

def hookBoundaryMap : List BoundaryEntry := [
  { fnName := "HookEvent",        traits := [.pure],            target := .typescript },
  { fnName := "HookOutput",       traits := [.pure, .io],       target := .typescript },
  { fnName := "HookResult",       traits := [.pure],            target := .typescript },
  { fnName := "HookRegistration", traits := [.pure],            target := .typescript }
]

end ClaudeCode.Hook
