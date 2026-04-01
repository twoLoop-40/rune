/-
  Domain/Query.lean — 쿼리 엔진 & 실행 루프
  Claude API 호출, 스트리밍, 도구 실행 오케스트레이션.
-/
import Common.Types

namespace ClaudeCode.Query

open ClaudeCode

-- ═══════════════════════════════════════════════
-- Query Source
-- ═══════════════════════════════════════════════

/-- 쿼리 소스. -/
inductive QuerySource where
  | repl           -- CLI REPL
  | sdk            -- SDK API
  | bridge         -- 리모트 브릿지
  | subagent       -- 서브에이전트 내부
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Thinking Configuration
-- ═══════════════════════════════════════════════

/-- 사고 모드 설정. -/
structure ThinkingConfig where
  enabled    : Bool := true
  budgetMs   : Option Nat := none
  deriving Repr

-- ═══════════════════════════════════════════════
-- Query Engine Configuration
-- ═══════════════════════════════════════════════

/-- 쿼리 엔진 설정. -/
structure QueryEngineConfig where
  cwd                : FilePath
  maxTurns           : Option Nat     := none
  maxBudgetUsd       : Option Nat     := none  -- cents
  verbose            : Bool           := false
  userSpecifiedModel : Option String  := none
  fallbackModel      : Option String  := none
  thinkingConfig     : ThinkingConfig := {}
  hasJsonSchema      : Bool           := false
  deriving Repr

-- ═══════════════════════════════════════════════
-- Finish Reason (API 응답)
-- ═══════════════════════════════════════════════

/-- API 응답 종료 이유. -/
inductive FinishReason where
  | endTurn     -- 모델이 자연 종료
  | stop        -- stop 시퀀스 매칭
  | maxTokens   -- 토큰 한도 초과
  | toolUse     -- 도구 사용 후 계속
  | error       -- API 에러
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Token Usage
-- ═══════════════════════════════════════════════

/-- 토큰 사용량. -/
structure TokenUsage where
  inputTokens       : Nat
  outputTokens      : Nat
  cacheCreation     : Nat := 0
  cacheRead         : Nat := 0
  deriving Repr

/-- 총 토큰. -/
def TokenUsage.total (u : TokenUsage) : Nat :=
  u.inputTokens + u.outputTokens

-- ═══════════════════════════════════════════════
-- Query Result (State Machine)
-- ═══════════════════════════════════════════════

/-- 쿼리 실행 결과 — Continue/Terminal 상태 머신. -/
inductive QueryResult where
  | terminal (finishReason : FinishReason) (usage : TokenUsage)
  | continue_ (nextTurnNeeded : Bool)
  deriving Repr

/-- terminal이면 루프 종료. -/
def QueryResult.isTerminal : QueryResult → Bool
  | .terminal _ _ => true
  | _             => false

-- ═══════════════════════════════════════════════
-- Streaming Events
-- ═══════════════════════════════════════════════

/-- 스트리밍 이벤트. -/
inductive StreamEvent where
  | contentBlockStart (index : Nat) (blockType : String)
  | contentBlockDelta (index : Nat) (delta : String)
  | contentBlockStop  (index : Nat)
  | messageStart
  | messageStop (finishReason : FinishReason)
  deriving Repr

-- ═══════════════════════════════════════════════
-- Context Compaction
-- ═══════════════════════════════════════════════

/-- 컨텍스트 압축 전략. -/
inductive CompactionStrategy where
  | summarize     -- 요약으로 대체
  | truncate      -- 오래된 메시지 삭제
  | collapse      -- 연속 도구 호출 접기
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Query Tracking
-- ═══════════════════════════════════════════════

/-- 쿼리 체인 추적 (서브에이전트 깊이). -/
structure QueryTracking where
  chainId : String
  depth   : Nat
  deriving Repr

/-- 서브에이전트 깊이 제한. -/
def maxQueryDepth : Nat := 10

/-- 깊이 초과 검사. -/
def QueryTracking.isAtMaxDepth (qt : QueryTracking) : Bool :=
  qt.depth ≥ maxQueryDepth

-- ═══════════════════════════════════════════════
-- Execution Flow
-- ═══════════════════════════════════════════════

/--
  쿼리 실행 흐름:
  1. buildSystemPrompt + context injection
  2. normalizeMessagesForAPI
  3. API request (streaming)
  4. StreamEvent 처리
  5. tool_use → StreamingToolExecutor (병렬)
  6. ToolResult 수집
  7. Continue → 메시지 추가 후 반복
  8. Terminal → 종료 + transcript 저장
-/
inductive ExecutionPhase where
  | buildPrompt
  | normalizeMessages
  | apiRequest
  | streamProcessing
  | toolExecution
  | resultCollection
  | continueLoop
  | terminate
  deriving Repr

-- ═══════════════════════════════════════════════
-- BoundaryMap
-- ═══════════════════════════════════════════════

def queryBoundaryMap : List BoundaryEntry := [
  { fnName := "QueryEngineConfig", traits := [.pure],            target := .typescript },
  { fnName := "QueryResult",       traits := [.pure, .provable], target := .typescript },
  { fnName := "TokenUsage",        traits := [.pure],            target := .typescript },
  { fnName := "StreamEvent",       traits := [.io, .concurrent], target := .typescript }
]

end ClaudeCode.Query
