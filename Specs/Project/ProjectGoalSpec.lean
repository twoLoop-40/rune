/-
  Project/ProjectGoalSpec.lean — Claude Code 프로젝트 목표 명세
  프로젝트의 도메인별 완성도를 추적하는 최상위 Spec.
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
  sorryCount  : Nat := 0
  deriving Repr

-- ═══════════════════════════════════════════════
-- Project Goals (All Domains)
-- ═══════════════════════════════════════════════

/-- 프로젝트 전체 목표. -/
def allGoals : List DomainGoal := [
  -- 공통 타입
  { domain := "Common.Types"
    description := "브랜드 ID, 타임스탬프, 에러, 파일 경로, JSON 값 등 공통 기초 타입"
    spec := .compiled
    impl := .deployed
    specFile := "Specs/Common/Types.lean"
    sorryCount := 0 },

  -- 도구 시스템
  { domain := "Domain.Tool"
    description := "Tool<Input,Output,Progress> 인터페이스. 40+ 빌트인 도구의 스키마, 권한, 동시성 모델"
    spec := .compiled
    impl := .deployed
    specFile := "Specs/Domain/Tool.lean"
    sorryCount := 1 },

  -- 태스크 시스템
  { domain := "Domain.Task"
    description := "백그라운드 태스크 상태 머신. 7종 유형, 5종 상태, 전이 규칙 증명"
    spec := .compiled
    impl := .deployed
    specFile := "Specs/Domain/Task.lean"
    sorryCount := 0 },

  -- 권한 시스템
  { domain := "Domain.Permission"
    description := "7종 권한 모드, 4종 결정(allow/deny/ask/passthrough), 분류기, 거부 추적"
    spec := .compiled
    impl := .deployed
    specFile := "Specs/Domain/Permission.lean"
    sorryCount := 0 },

  -- 메시지 시스템
  { domain := "Domain.Message"
    description := "메시지 타입 계층, 직렬화, 트랜스크립트 트리, 큐 시스템"
    spec := .compiled
    impl := .deployed
    specFile := "Specs/Domain/Message.lean"
    sorryCount := 0 },

  -- 쿼리 엔진
  { domain := "Domain.Query"
    description := "Claude API 호출 루프. 스트리밍, 도구 실행, 컨텍스트 압축, 토큰 추적"
    spec := .compiled
    impl := .deployed
    specFile := "Specs/Domain/Query.lean"
    sorryCount := 0 },

  -- 앱 상태
  { domain := "Domain.State"
    description := "AppState 루트. DeepImmutable. MCP, 플러그인, 태스크, 팀, UI 상태"
    spec := .compiled
    impl := .deployed
    specFile := "Specs/Domain/State.lean"
    sorryCount := 0 },

  -- 훅 시스템
  { domain := "Domain.Hook"
    description := "17종 이벤트 훅. 콜백, 비동기, 권한 위임"
    spec := .compiled
    impl := .deployed
    specFile := "Specs/Domain/Hook.lean"
    sorryCount := 0 },

  -- 플러그인 시스템
  { domain := "Domain.Plugin"
    description := "플러그인 라이프사이클, 매니페스트, MCP/LSP 서버, 21종 에러 유형"
    spec := .compiled
    impl := .deployed
    specFile := "Specs/Domain/Plugin.lean"
    sorryCount := 0 },

  -- 명령어 시스템
  { domain := "Domain.Command"
    description := "100+ 슬래시 커맨드. prompt/local/localJsx 3종, 레지스트리"
    spec := .compiled
    impl := .deployed
    specFile := "Specs/Domain/Command.lean"
    sorryCount := 0 },

  -- 브릿지
  { domain := "Domain.Bridge"
    description := "리모트 컨트롤 프로토콜. WebSocket 기반 세션 제어"
    spec := .compiled
    impl := .deployed
    specFile := "Specs/Domain/Bridge.lean"
    sorryCount := 0 }
]

-- ═══════════════════════════════════════════════
-- Progress Snapshot
-- ═══════════════════════════════════════════════

/-- 진행 상태 요약. -/
structure ProgressSnapshot where
  totalDomains    : Nat
  specCompiled    : Nat
  specProved      : Nat
  implDeployed    : Nat
  totalSorries    : Nat
  deriving Repr

/-- 현재 진행 상태 계산. -/
def progressSnapshot : ProgressSnapshot :=
  let total := allGoals.length
  let compiled := allGoals.filter (fun g => g.spec == .compiled || g.spec == .proved || g.spec == .tested) |>.length
  let proved := allGoals.filter (fun g => g.spec == .proved || g.spec == .tested) |>.length
  let deployed := allGoals.filter (fun g => g.impl == .deployed) |>.length
  let sorries := allGoals.foldl (fun acc g => acc + g.sorryCount) 0
  { totalDomains := total
    specCompiled := compiled
    specProved := proved
    implDeployed := deployed
    totalSorries := sorries }

-- ═══════════════════════════════════════════════
-- Gap Analysis
-- ═══════════════════════════════════════════════

/-- Spec이 아직 proved가 아닌 도메인. -/
def specGaps : List DomainGoal :=
  allGoals.filter (fun g => g.spec != .proved && g.spec != .tested)

/-- sorry가 남은 도메인. -/
def sorryGaps : List DomainGoal :=
  allGoals.filter (fun g => g.sorryCount > 0)

end ClaudeCode.ProjectGoal
