/-
  Project/ProjectGoalSpec.lean — rune 프로젝트 목표 명세
  LLM-agnostic agent engine의 도메인별 완성도 추적.

  Milestone 1 (vertical slice): API + Bash/FileRead/FileWrite + CLI
  → "cargo run -- '질문'" 으로 실제 동작하는 에이전트
-/
import Common.Types

namespace ClaudeCode.ProjectGoal

open ClaudeCode

-- ═══════════════════════════════════════════════
-- Goal Stage (성숙도)
-- ═══════════════════════════════════════════════

/-- Spec 성숙도. -/
inductive SpecStage where
  | notStarted
  | drafted       -- 초안 작성
  | compiled      -- lake build 통과
  | proved        -- sorry 0
  | tested        -- 구현 검증됨
  deriving Repr, BEq

/-- 구현 성숙도. -/
inductive ImplStage where
  | notStarted
  | inProgress
  | implDone
  | tested
  | deployed
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Domain Goal
-- ═══════════════════════════════════════════════

/-- 도메인 목표. -/
structure DomainGoal where
  domain      : String
  description : String
  spec        : SpecStage
  impl        : ImplStage
  specFile    : Option String := none
  rustFile    : Option String := none
  sorryCount  : Nat := 0
  milestone   : Nat := 0    -- 0=foundation, 1=vertical-slice, 2=expand, 3=polish
  deriving Repr

-- ═══════════════════════════════════════════════
-- Milestone 0: Foundation (완료)
-- Lean Spec + Rust 타입 + 엔진 뼈대
-- ═══════════════════════════════════════════════

-- ═══════════════════════════════════════════════
-- Milestone 1: Vertical Slice (현재 목표)
-- API provider + 도구 3개 + CLI = 동작하는 에이전트
-- ═══════════════════════════════════════════════

-- ═══════════════════════════════════════════════
-- Milestone 2: Expand
-- 나머지 빌트인 도구 + TUI + 세션 관리
-- ═══════════════════════════════════════════════

-- ═══════════════════════════════════════════════
-- Milestone 3: Polish
-- MCP/Plugin + Bridge + gdd/poincare 통합
-- ═══════════════════════════════════════════════

def allGoals : List DomainGoal := [
  -- ════════ Milestone 0: Foundation (완료) ════════

  { domain := "Common.Types"
    description := "브랜드 ID, 타임스탬프, 에러, JSON 등 공통 타입"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Common/Types.lean"
    rustFile := "crates/core/src/types.rs"
    milestone := 0 },

  { domain := "Core.LlmProvider"
    description := "LLM-agnostic provider trait. stream() + supports_model()"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Query.lean"
    rustFile := "crates/core/src/llm.rs"
    milestone := 0 },

  { domain := "Engine.QueryEngine"
    description := "메인 실행 루프. stream → tool → continue/terminal"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Query.lean"
    rustFile := "crates/engine/src/query_engine.rs"
    milestone := 0 },

  { domain := "Engine.ToolExecutor"
    description := "도구 병렬/순차 실행. 권한 + 훅 통합"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Tool.lean"
    rustFile := "crates/engine/src/executor.rs"
    milestone := 0 },

  { domain := "Tools.Trait"
    description := "Tool trait + ToolRegistry + PermissionChecker + HookRunner"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Tool.lean"
    rustFile := "crates/tools/src/lib.rs"
    milestone := 0 },

  { domain := "Domain.Task"
    description := "태스크 상태 머신. Rust typestate로 전이 규칙 컴파일타임 강제"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Task.lean"
    rustFile := "crates/core/src/task.rs"
    milestone := 0 },

  { domain := "Domain.Permission"
    description := "7종 권한 모드, 4종 결정, 분류기, 거부 추적"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Permission.lean"
    rustFile := "crates/core/src/permission.rs"
    milestone := 0 },

  -- ════════ Milestone 1: Vertical Slice (완료 ✅) ════════

  { domain := "API.Claude"
    description := "Anthropic Claude API provider. SSE 스트리밍, 인증, 재시도"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Query.lean"
    rustFile := "crates/api/src/claude.rs"
    milestone := 1 },

  { domain := "Tools.Bash"
    description := "셸 명령 실행 도구. 타임아웃, 출력 캡처"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Tool.lean"
    rustFile := "crates/tools/src/builtin/bash.rs"
    milestone := 1 },

  { domain := "Tools.FileRead"
    description := "파일 읽기 도구. 라인 번호, 범위 지정"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Tool.lean"
    rustFile := "crates/tools/src/builtin/file_read.rs"
    milestone := 1 },

  { domain := "Tools.FileWrite"
    description := "파일 쓰기 도구. 생성/덮어쓰기"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Tool.lean"
    rustFile := "crates/tools/src/builtin/file_write.rs"
    milestone := 1 },

  { domain := "CLI.Main"
    description := "최소 CLI 진입점. stdin → engine → stdout"
    spec := .compiled, impl := .implDone
    rustFile := "crates/cli/src/main.rs"
    milestone := 1 },

  -- ════════ Milestone 2: Expand ════════

  { domain := "Tools.FileEdit"
    description := "파일 편집 도구. 문자열 치환"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Tool.lean"
    rustFile := "crates/tools/src/builtin/file_edit.rs"
    milestone := 2 },

  { domain := "Tools.Glob"
    description := "파일 패턴 검색"
    spec := .compiled, impl := .implDone
    rustFile := "crates/tools/src/builtin/glob.rs"
    milestone := 2 },

  { domain := "Tools.Grep"
    description := "파일 내용 검색 (ripgrep)"
    spec := .compiled, impl := .implDone
    rustFile := "crates/tools/src/builtin/grep.rs"
    milestone := 2 },

  { domain := "Tools.Agent"
    description := "서브에이전트 생성/관리"
    spec := .compiled, impl := .notStarted
    specFile := "Specs/Domain/Agent.lean"
    milestone := 3 },

  { domain := "CLI.REPL"
    description := "대화형 REPL. 슬래시 커맨드, 세션 저장/복원"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/CLI.lean"
    rustFile := "crates/cli/src/main.rs"
    milestone := 2 },

  { domain := "Session.Management"
    description := "세션 저장/복원/이력 (~/.rune/sessions/)"
    spec := .compiled, impl := .implDone
    specFile := "Specs/Domain/Session.lean"
    rustFile := "crates/cli/src/session.rs"
    milestone := 2 },

  { domain := "Domain.Command"
    description := "슬래시 커맨드 레지스트리 실행"
    spec := .compiled, impl := .inProgress
    specFile := "Specs/Domain/Command.lean"
    milestone := 2 },

  -- ════════ Milestone 3: Polish ════════

  { domain := "TUI.Ratatui"
    description := "ratatui 터미널 UI. 스트리밍, 도구 진행, 마크다운 렌더링"
    spec := .drafted, impl := .notStarted
    milestone := 3 },

  { domain := "Plugin.MCP"
    description := "MCP 서버 관리, 동적 도구 로드"
    spec := .compiled, impl := .notStarted
    specFile := "Specs/Domain/Plugin.lean"
    milestone := 3 },

  { domain := "Bridge.Remote"
    description := "WebSocket 리모트 컨트롤"
    spec := .compiled, impl := .notStarted
    specFile := "Specs/Domain/Bridge.lean"
    milestone := 3 },

  { domain := "Integration.GDD"
    description := "gdd-unified 엔진 통합 (스킬→네이티브)"
    spec := .notStarted, impl := .notStarted
    milestone := 3 },

  { domain := "Integration.Poincare"
    description := "poincare-retrain 엔진 통합"
    spec := .notStarted, impl := .notStarted
    milestone := 3 }
]

-- ═══════════════════════════════════════════════
-- Progress Snapshot
-- ═══════════════════════════════════════════════

structure ProgressSnapshot where
  totalDomains    : Nat
  specCompiled    : Nat
  specProved      : Nat
  implDone        : Nat
  totalSorries    : Nat
  currentMilestone : Nat
  deriving Repr

def progressSnapshot : ProgressSnapshot :=
  let total := allGoals.length
  let compiled := allGoals.filter (fun g => g.spec == .compiled || g.spec == .proved || g.spec == .tested) |>.length
  let proved := allGoals.filter (fun g => g.spec == .proved || g.spec == .tested) |>.length
  let done := allGoals.filter (fun g => g.impl == .implDone || g.impl == .tested || g.impl == .deployed) |>.length
  let sorries := allGoals.foldl (fun acc g => acc + g.sorryCount) 0
  { totalDomains := total
    specCompiled := compiled
    specProved := proved
    implDone := done
    totalSorries := sorries
    currentMilestone := 2 }

-- ═══════════════════════════════════════════════
-- Gap Analysis
-- ═══════════════════════════════════════════════

/-- 현재 마일스톤의 미완료 목표. -/
def currentGaps : List DomainGoal :=
  allGoals.filter (fun g => g.milestone == 2 && g.impl == .notStarted)

/-- sorry가 남은 도메인. -/
def sorryGaps : List DomainGoal :=
  allGoals.filter (fun g => g.sorryCount > 0)

end ClaudeCode.ProjectGoal
