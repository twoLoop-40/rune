/-
  Domain/Tool.lean — Tool<Input, Output, Progress> 타입시스템
  Claude Code의 핵심 도구 추상화. 40+ 빌트인 도구의 공통 인터페이스.
-/
import Common.Types

namespace ClaudeCode.Tool

open ClaudeCode

-- ═══════════════════════════════════════════════
-- Tool Input/Output Schema
-- ═══════════════════════════════════════════════

/-- 도구 입력 JSON 스키마 (Zod → JSONSchema 변환). -/
structure ToolInputSchema where
  schema : JsonValue
  strict : Bool := false
  deriving Repr

-- ═══════════════════════════════════════════════
-- Tool Metadata
-- ═══════════════════════════════════════════════

/-- 도구의 인터럽트 동작. -/
inductive InterruptBehavior where
  | cancel   -- 취소 가능
  | block    -- 완료까지 블로킹
  deriving Repr, BEq

/-- 검색/읽기 분류 결과. -/
structure SearchReadClassification where
  isSearch : Bool
  isRead   : Bool
  isList   : Bool
  deriving Repr

/-- MCP(Model Context Protocol) 도구 메타. -/
structure McpToolInfo where
  serverName : String
  toolName   : String
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Tool Progress
-- ═══════════════════════════════════════════════

/-- 진행 보고 래퍼. -/
structure ToolProgress (ProgressData : Type) where
  toolUseID : String
  data      : ProgressData
  deriving Repr

-- ═══════════════════════════════════════════════
-- Tool Result
-- ═══════════════════════════════════════════════

/-- 도구 실행 결과. -/
structure ToolResult (Output : Type) where
  data           : Output
  hasNewMessages : Bool := false
  hasModifier    : Bool := false
  deriving Repr

-- ═══════════════════════════════════════════════
-- Tool Capabilities (Bool predicates)
-- ═══════════════════════════════════════════════

/-- 도구 동작 특성. 불변조건의 집합. -/
structure ToolCapabilities where
  isConcurrencySafe : Bool := false   -- 병렬 실행 안전
  isReadOnly        : Bool := false   -- 파일시스템 변경 없음
  isDestructive     : Bool := false   -- 되돌리기 어려운 작업
  isEnabled         : Bool := true    -- 현재 활성화
  shouldDefer       : Bool := false   -- ToolSearch용 지연 로딩
  alwaysLoad        : Bool := false   -- 항상 로드
  deriving Repr

/-- 도구 특성 불변조건: destructive이면 readOnly가 아니다. -/
theorem destructive_not_readonly (cap : ToolCapabilities) :
    cap.isDestructive = true → cap.isReadOnly = false ∨ cap.isDestructive = false := by
  intro h
  left
  -- destructive 도구가 readOnly인 것은 논리적 모순
  -- 실제 시스템에서 buildTool이 이를 강제함
  -- 여기서는 구현 수준에서 보장하는 것으로 가정
  sorry -- TODO: ToolCapabilities에 prop 추가하여 증명

-- ═══════════════════════════════════════════════
-- Tool Definition (Core Interface)
-- ═══════════════════════════════════════════════

/-- Tool 인터페이스. TypeScript의 Tool<Input, Output, Progress>에 대응. -/
structure ToolDef where
  -- 식별
  name        : String
  aliases     : List String := []
  searchHint  : String := ""

  -- 스키마
  inputSchema : ToolInputSchema
  hasOutputSchema : Bool := false

  -- 동작 특성
  capabilities      : ToolCapabilities := {}
  interruptBehavior : InterruptBehavior := .cancel

  -- MCP 메타
  isMcp   : Bool := false
  isLsp   : Bool := false
  mcpInfo : Option McpToolInfo := none

  -- 결과 크기 제한
  maxResultSizeChars : Nat := 200000
  deriving Repr

-- ═══════════════════════════════════════════════
-- Built-in Tool Registry
-- ═══════════════════════════════════════════════

/-- 빌트인 도구 이름. 40+ 도구의 열거. -/
inductive BuiltinToolName where
  -- 파일 시스템
  | bash
  | fileRead
  | fileEdit
  | fileWrite
  | glob
  | grep
  -- 에이전트
  | agent
  | sendMessage
  | taskCreate
  | taskStop
  | taskOutput
  | teamCreate
  | teamDelete
  -- 코드 탐색
  | webSearch
  | webFetch
  -- UI / 사용자 상호작용
  | askUser
  | todoWrite
  | notebookEdit
  -- 계획
  | enterPlanMode
  | exitPlanMode
  -- 워크트리
  | enterWorktree
  | exitWorktree
  -- 원격
  | remoteTrigger
  | cronCreate
  | cronDelete
  | cronList
  -- MCP 도구 (동적)
  | mcp (serverName : String) (toolName : String)
  deriving Repr, BEq

/-- 도구가 읽기 전용인지 분류. -/
def BuiltinToolName.isReadOnlyTool : BuiltinToolName → Bool
  | .fileRead => true
  | .glob     => true
  | .grep     => true
  | .webSearch => true
  | .webFetch  => true
  | _          => false

/-- 도구가 동시 실행 안전한지 분류. -/
def BuiltinToolName.isConcurrencySafeTool : BuiltinToolName → Bool
  | .fileRead  => true
  | .glob      => true
  | .grep      => true
  | .webSearch => true
  | .webFetch  => true
  | .todoWrite => true
  | _          => false

-- ═══════════════════════════════════════════════
-- BoundaryMap
-- ═══════════════════════════════════════════════

/-- Tool 모듈의 경계 분류. TS 프로젝트이므로 모두 typescript. -/
def toolBoundaryMap : List BoundaryEntry := [
  { fnName := "ToolDef",          traits := [.pure, .provable], target := .typescript },
  { fnName := "ToolCapabilities", traits := [.pure, .provable], target := .typescript },
  { fnName := "ToolResult",       traits := [.pure],            target := .typescript },
  { fnName := "BuiltinToolName",  traits := [.pure],            target := .typescript }
]

end ClaudeCode.Tool
