import LongTermMemory.MemorySchema

set_option autoImplicit true
open LongTermMemory.MemorySchema

-- @spec Specs/Common/Types.lean
-- @spec Specs/Domain/Tool.lean
-- @spec Specs/Domain/Task.lean
-- @spec Specs/Domain/Permission.lean
-- @spec Specs/Domain/Query.lean
-- @spec Specs/Domain/State.lean

namespace LongTermMemory.Domain.ClaudeCodeArchitecture

-- ① Spec↔Code 연결
def specLinks : List SpecCodeLink := [
  { spec := "Specs/Common/Types.lean"
    code := "types/"
    invariant := "SessionId, AgentId, UUID, Timestamp 등 브랜드 타입 1:1 매핑" },
  { spec := "Specs/Domain/Tool.lean"
    code := "Tool.ts"
    invariant := "Tool<Input,Output,Progress> 제네릭 인터페이스. ToolCapabilities 불변조건" },
  { spec := "Specs/Domain/Task.lean"
    code := "Task.ts"
    invariant := "TaskStatus 상태머신. ValidTransition으로 전이 규칙 증명. no_transition_from_terminal" },
  { spec := "Specs/Domain/Permission.lean"
    code := "types/permissions.ts"
    invariant := "7종 PermissionMode, 4종 PermissionDecision. bypass_always_allows 정리" },
  { spec := "Specs/Domain/Query.lean"
    code := "QueryEngine.ts + query.ts"
    invariant := "Continue/Terminal 상태머신. TokenUsage 추적. maxQueryDepth=10" },
  { spec := "Specs/Domain/State.lean"
    code := "state/AppStateStore.ts"
    invariant := "DeepImmutable AppState. setAppState 함수형 업데이트" },
  { spec := "Specs/Domain/Hook.lean"
    code := "types/hooks.ts"
    invariant := "17종 HookEvent. PreToolUse block → 실행 차단 증명" },
  { spec := "Specs/Domain/Plugin.lean"
    code := "types/plugin.ts"
    invariant := "PluginLifecycle 상태머신. 21종 에러 유형" },
  { spec := "Specs/Domain/Command.lean"
    code := "commands.ts"
    invariant := "100+ 슬래시 커맨드. prompt/local/localJsx 3종" },
  { spec := "Specs/Domain/Bridge.lean"
    code := "bridge/"
    invariant := "WebSocket 리모트 세션 제어 프로토콜" },
  { spec := "Specs/Domain/Agent.lean"
    code := "tools/AgentTool/"
    invariant := "BuiltIn/Custom/Plugin 3종 에이전트. ForkedAgentParams" },
  { spec := "Specs/Domain/Model.lean"
    code := "utils/model/"
    invariant := "API Provider, ModelConfig, CostTrackerState" }
]

-- ② 시행착오
def trials : List TrialRecord := [
  { topic := "Lean 4 inductive 키워드 충돌"
    tried := "partial, implemented, complete, done, prefix를 생성자 이름으로 사용"
    result := "모두 Lean 4 예약어. lake build 실패"
    lesson := "inProgress, implDone, regionPrefix로 대체. Lean 4 키워드 목록 숙지 필수" },
  { topic := "lakefile srcDir와 import 경로"
    tried := "srcDir := 'Specs' 설정 후 import Specs.Common.Types"
    result := "unknown module prefix 'Specs' 에러"
    lesson := "srcDir가 이미 루트이므로 import Common.Types로 작성해야 함" },
  { topic := "Poincare Spec-Code 연결"
    tried := "@spec 태그 없이 Poincare run"
    result := "Spec 노드와 TS 노드 거리가 ~4.0으로 멂 (약한 연결)"
    lesson := "Lean 파일에 -- @spec, TS 파일에 // @spec 태그를 추가해야 강한 edge 생성" }
]

-- ③ 의미 기억
def architectureKnowledge : SemanticMemory :=
  { fact := "Claude Code 핵심 아키텍처: QueryEngine(API 루프) → StreamingToolExecutor(병렬 도구) → Tool.checkPermissions → Tool.call. AppState가 단일 진실 원천. 모든 상태는 setAppState 함수형 업데이트."
    domain := some "architecture"
    source := some "Specs/Domain/Query.lean + Specs/Domain/State.lean"
    stability := ⟨90, by omega⟩
    maturity := .consolidated }

def rustRewritePlan : SemanticMemory :=
  { fact := "Rust 재작성 전략: Task→typestate, Permission→enum, Query→tokio async, Tool→trait, UI→ratatui. BoundaryMap 전체가 .typescript→.rust 전환 대상. 10만줄+ 풀 재작성 프로젝트."
    domain := some "rust-rewrite"
    source := some "사용자 대화 2026-04-01"
    stability := ⟨85, by omega⟩
    maturity := .raw }

-- ③-2 Rust 엔진 아키텍처 (2026-04-02 추가)
def runeEngineArch : SemanticMemory :=
  { fact := "rune (Rust 재작성) 엔진: LlmProvider trait(LLM-agnostic) -> QueryEngine(stream->tool->loop) -> ToolExecutor(parallel/seq) -> Tool trait. StreamHandler로 UI 분리. 6 crate workspace: core(286+ 타입), engine, tools, api, tui, server."
    domain := some "rune-architecture"
    source := some "rune 구현 2026-04-02"
    stability := ⟨90, by omega⟩
    maturity := .consolidated }

def runeProjectMeta : SemanticMemory :=
  { fact := "프로젝트명 rune (Rust+Engine). GitHub: twoLoop-40/rune. 기존 claude-code(TS)의 Lean Spec 기반 풀 재작성. 사용자 비전: gdd-unified + poincare-retrain 통합 AI 개발 플랫폼."
    domain := some "project-meta"
    source := some "사용자 대화 2026-04-02"
    stability := ⟨85, by omega⟩
    maturity := .consolidated }

theorem all_links_valid : specLinks.all SpecCodeLink.isValid = true := by native_decide

end LongTermMemory.Domain.ClaudeCodeArchitecture
