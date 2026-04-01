import LongTermMemory.MemorySchema

set_option autoImplicit true
open LongTermMemory.MemorySchema

-- @spec Specs/Common/Types.lean
-- @spec Specs/Domain/Tool.lean
-- @spec Specs/Domain/Task.lean
-- @spec Specs/Domain/Permission.lean
-- @spec Specs/Domain/Query.lean
-- @spec Specs/Domain/State.lean

namespace LongTermMemory.Sessions.Session_2026_04_01_1

def «meta» : SessionMeta :=
  { date := "2026-04-01"
    commits := []
    files := 35 }

def ctx : Context :=
  { project := some "claude-code"
    domain := some "type-system-formalization"
    technology := ["Lean 4", "TypeScript", "Rust (Poincare)"]
    tags := ["gdd-unified", "formal-spec", "poincare", "rust-rewrite"] }

-- ═══════════════════════════════════════════════
-- 에피소드 기억
-- ═══════════════════════════════════════════════

def episodes : List EpisodicMemory := [
  { what := "Claude Code TS 프로젝트를 Lean 4 타입시스템으로 정형화. /gdd-unified 워크플로우로 23개 Lean Spec 파일 작성, lake build 26/26 통과."
    when_ := "2026-04-01T14:00"
    context := ctx
    outcome := some "Common.Types + 11 Domain Specs (Tool, Task, Permission, Message, Query, State, Hook, Plugin, Command, Bridge + 12 추가) 완성" },
  { what := "Poincare 임베딩 학습 완료. 5974 노드, 16601 pairs. Spec 286 노드 vs TS 5688 노드. 갭 분석으로 커버리지 약한 TS 모듈 식별."
    when_ := "2026-04-01T15:30"
    context := ctx
    outcome := some "services/analytics, services/autoDream, utils/background, utils/teleport, components/ 등이 Spec 미커버" },
  { what := "사용자가 Rust로 풀 재작성 의사 표명. Lean Spec을 기반으로 Rust 프로젝트 시작 예정. 기존 TS 10만줄+ 규모."
    when_ := "2026-04-01T16:00"
    context := ctx
    outcome := some "풀 재작성 방향 확정. BoundaryMap에서 모든 함수가 .typescript이지만 .rust로 전환 예정" }
]

-- ═══════════════════════════════════════════════
-- 의미 기억
-- ═══════════════════════════════════════════════

def semantics : List SemanticMemory := [
  { fact := "Claude Code는 Bun 기반 TS/TSX CLI 앱. main.tsx 800KB, query.ts 68KB 등 총 10만줄+. 500개+ 파일, 1200+ 타입 정의."
    domain := some "architecture"
    source := some "README.md + 코드 탐색"
    stability := ⟨90, by omega⟩
    maturity := .consolidated },
  { fact := "Lean 4 키워드 회피 필수: partial, implemented, complete, done, prefix 등은 inductive 생성자로 사용 불가. implDone, inProgress, regionPrefix 등으로 대체."
    domain := some "lean4"
    source := some "lake build 에러에서 학습"
    stability := ⟨95, by omega⟩
    maturity := .consolidated },
  { fact := "Lean 4 Spec import 경로: srcDir 설정 시 해당 디렉토리가 루트. Specs.Common.Types가 아니라 Common.Types로 import."
    domain := some "lean4"
    source := some "lake build 에러에서 학습"
    stability := ⟨95, by omega⟩
    maturity := .consolidated },
  { fact := "Poincare 임베딩에서 Spec 노드와 TS 소스 노드의 거리가 약 4.0. @spec 태그 없이는 Spec-Code 연결이 약함. 태그 추가로 연결 강화 필요."
    domain := some "poincare"
    source := some "poincare search 결과"
    stability := ⟨80, by omega⟩
    maturity := .raw },
  { fact := "사용자는 Rust 풀 재작성을 원함. Task 상태머신 → typestate, Permission → enum, Query 엔진 → tokio async, UI → ratatui 매핑 계획."
    domain := some "rust-rewrite"
    source := some "사용자 대화"
    stability := ⟨85, by omega⟩
    maturity := .raw }
]

theorem episode_count : episodes.length = 3 := by native_decide
theorem semantic_count : semantics.length = 5 := by native_decide

end LongTermMemory.Sessions.Session_2026_04_01_1
