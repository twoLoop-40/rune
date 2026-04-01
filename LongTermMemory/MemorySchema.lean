-- MemorySchema v7: v3 (상태 모델 + 불변량) + v6 (뇌과학 4대원칙 그래프)
--
-- 이 파일은 모든 프로젝트에서 공유하는 canonical 스키마.
-- 위치: long-term-memory/MemorySchema.lean (이 스킬에 포함)
-- 프로젝트 초기화 시 LongTermMemory/MemorySchema.lean으로 복사됨.
--
-- 근원 문서: FOUNDATIONS.md (이 스킬에 포함)
-- 4대 원칙: 분산성, 가소성, 성숙성, 모순대처
--
-- v3: 상태 전이 시스템, 에피소드/의미/절차 기억, Context, MemoryDay
-- v6: SpecCodeLink, Maturity, Stability, MemoryState, Contradiction
-- v7: 두 스키마 병합. Session/Day 파일 호환 유지.
--
-- 변경 시 이 canonical 파일을 수정하고, 각 프로젝트에 동기화.

set_option autoImplicit true

namespace LongTermMemory.MemorySchema

-- ════════════════════════════════════════════════════════════════
-- §1. Core Types
-- ════════════════════════════════════════════════════════════════

abbrev DateStr := String
abbrev FilePath := String
abbrev Timestamp := String

-- ════════════════════════════════════════════════════════════════
-- §2. 분산성 (Distributedness)
--
-- 뇌: 엔그램 복합체 (Tonegawa) — 기억이 여러 영역에 분산
-- 시스템: Spec↔Memory↔Code 삼각 연결
-- ════════════════════════════════════════════════════════════════

/-- Spec↔Code 연결 — 장기기억의 핵심 단위 (엔그램 복합체).
    하나의 지식이 스펙, 코드, 기억 세 곳에 분산되어 존재한다. -/
structure SpecCodeLink where
  spec      : FilePath       -- Specs/ 경로
  code      : FilePath       -- 구현 파일 경로
  invariant : String         -- 어떤 불변식/지식이 연결되는지
  deriving Repr, BEq

/-- 시행착오 기록 — 시도/결과/교훈을 구조화 -/
structure TrialRecord where
  topic     : String         -- 무엇에 대한 시행착오
  tried     : String         -- 시도한 것
  result    : String         -- 결과 (성공/실패/발견)
  lesson    : String         -- 배운 것 (theorem으로 증명 가능해야)
  deriving Repr

/-- 시행착오 기억 — 가설→실험→결과→결론 (Session 파일용) -/
structure TrialMemory where
  hypothesis : String        -- 가설
  experiment : String        -- 실험 방법
  result     : String        -- 결과
  conclusion : String        -- 결론/다음 단계
  deriving Repr

-- ════════════════════════════════════════════════════════════════
-- §3. 성숙성 (Maturity / Consolidation)
--
-- 뇌: 에피소드→의미 전환 (Tulving), CLS 이론 (McClelland)
-- 시스템: 커밋(raw) → Domain(consolidated) → Common(generalized)
-- Poincaré: 경계(구체) → 중심(추상)
-- ════════════════════════════════════════════════════════════════

/-- 기억의 성숙 단계 -/
inductive Maturity where
  | raw          -- 커밋 직후 (에피소드)
  | consolidated -- Domain에 통합 (의미)
  | generalized  -- Common으로 승격 (도메인 횡단)
  deriving BEq, DecidableEq, Repr

/-- 성숙은 단방향 -/
def validMaturation : Maturity → Maturity → Bool
  | .raw, .consolidated          => true
  | .raw, .generalized           => true   -- 스키마 일치 시 빠른 경로
  | .consolidated, .generalized  => true
  | _, _                         => false

theorem raw_to_consolidated : validMaturation .raw .consolidated = true := by decide
theorem no_regression : validMaturation .generalized .raw = false := by decide

-- ════════════════════════════════════════════════════════════════
-- §4. 가소성 (Plasticity)
--
-- 뇌: LTP/LTD (Hebb), 재공고화 (Nader et al., 2000)
-- 시스템: Stability로 강도 추적, 인출 시 수정 가능
-- ════════════════════════════════════════════════════════════════

/-- 안정성 — 에빙하우스 S 파라미터 -/
structure Stability where
  val : Nat
  h   : val ≤ 100 := by omega
  deriving Repr

/-- 기억 상태 — 재공고화 모델
    인출된 기억은 labile이 되어 수정 가능 -/
inductive MemoryState where
  | stable         -- 안정
  | labile         -- 불안정 (인출 후)
  | reconsolidated -- 재공고화됨
  deriving BEq, DecidableEq, Repr

/-- 재공고화 전이 -/
def validReconsolidation : MemoryState → MemoryState → Bool
  | .stable, .labile                 => true
  | .labile, .reconsolidated         => true
  | .reconsolidated, .stable         => true
  | _, _                             => false

theorem retrieval_destabilizes : validReconsolidation .stable .labile = true := by decide

-- ════════════════════════════════════════════════════════════════
-- §5. 모순 대처 (Contradiction Tolerance)
--
-- 뇌: 간섭 이론, 능동적 망각 (Davis & Zhong), 스키마 수용 (Piaget)
-- 시스템: 모순 태깅 → 두 버전 공존 → 증거 축적 → 해소
-- ════════════════════════════════════════════════════════════════

/-- 모순의 상태 -/
inductive ContradictionStatus where
  | detected       -- 감지 (두 버전 공존)
  | investigating  -- 조사 중
  | resolved       -- 해소 (하나가 승리)
  | accommodated   -- 수용 (둘 다 맞음, 조건부)
  deriving BEq, DecidableEq, Repr

/-- 모순 기록 -/
structure Contradiction where
  topic      : String
  versionA   : String
  versionB   : String
  status     : ContradictionStatus
  resolution : Option String := none
  deriving Repr

-- ════════════════════════════════════════════════════════════════
-- §6. 기억 타입 (v3 + v6 병합)
-- ════════════════════════════════════════════════════════════════

/-- 세션 메타데이터 (v3) -/
structure SessionMeta where
  date    : DateStr
  commits : List String := []
  files   : Nat := 0
  deriving Repr, BEq

/-- 컨텍스트 — 기억의 배경 정보 -/
structure Context where
  project      : Option String     := none
  domain       : Option String     := none
  technology   : List String       := []
  relatedFiles : List String       := []
  tags         : List String       := []
  deriving Repr, Inhabited

def emptyContext : Context := {}

/-- 에피소드 기억 — 사건 기록 -/
structure EpisodicMemory where
  what             : String
  when_            : Timestamp
  where_           : Option String := none
  context          : Context       := {}
  emotionalValence : Option Int    := none
  outcome          : Option String := none
  deriving Repr

/-- 의미 기억 (Tulving) — Domain 파일의 핵심 -/
structure SemanticMemory where
  fact            : String
  domain          : Option String    := none
  source          : Option String    := none
  stability       : Stability        := ⟨50, by omega⟩
  maturity        : Maturity         := .consolidated
  relatedConcepts : List String      := []
  deriving Repr

/-- 절차 기억 (Tulving) — Common 파일의 핵심 -/
structure ProceduralMemory where
  skill         : String
  steps         : List String
  prerequisites : List String := []
  commonErrors  : List String := []
  context       : Context     := {}
  maturity      : Maturity    := .generalized
  deriving Repr

/-- 프라이밍 기억 — 트리거/응답 패턴 -/
structure PrimingMemory where
  trigger   : String
  response  : String
  rationale : Option String := none
  deriving Repr

/-- 태스크 상태 -/
inductive TaskStatus where
  | pending | inProgress | completed | blocked
  deriving Repr, BEq, DecidableEq

/-- 태스크 기억 (의존 타입) -/
inductive TaskMemory : TaskStatus → Type where
  | completed : (description : String) → (context : Context) →
      (completedAt : Timestamp) → TaskMemory .completed
  | active : (description : String) → (context : Context) →
      (status : TaskStatus) → TaskMemory status
  | blocked : (description : String) → (context : Context) →
      (blockedBy : String) → TaskMemory .blocked

def TaskMemory.getDescription {s : TaskStatus} : TaskMemory s → String
  | .completed desc _ _ => desc
  | .active desc _ _    => desc
  | .blocked desc _ _   => desc

structure Uncertainty where
  topic               : String
  knownAspects        : List String := []
  unknownAspects      : List String := []
  investigationNeeded : Option String := none
  deriving Repr

structure SomeTask where
  status : TaskStatus
  task   : TaskMemory status

-- ════════════════════════════════════════════════════════════════
-- §6b. TypeForge 프로젝트 전용 타입
-- ════════════════════════════════════════════════════════════════

/-- 기억의 종류 -/
inductive MemoryKind where
  | episodic   : MemoryKind
  | semantic   : MemoryKind
  | procedural : MemoryKind
  | task       : MemoryKind
  deriving BEq, DecidableEq, Repr

/-- TypeForge 개발 단계 -/
inductive Phase where
  | dataBackend   : Phase
  | dashboard     : Phase
  | auth          : Phase
  | backendFull   : Phase
  | frontend      : Phase
  deriving BEq, DecidableEq, Repr

/-- MemoryDay — 일일 기억 컨테이너 -/
structure MemoryDay where
  date          : DateStr
  summary       : Option String       := none
  sessionFiles  : List String         := []
  domainUpdates : List String         := []
  episodes      : List EpisodicMemory := []
  semantics     : List SemanticMemory := []
  procedures    : List ProceduralMemory := []
  primings      : List PrimingMemory  := []
  tasks         : List SomeTask       := []
  uncertainties : List Uncertainty    := []
  phase         : Option Phase        := none

def emptyDay (d : DateStr) : MemoryDay := { date := d }

-- ════════════════════════════════════════════════════════════════
-- §7. 커밋 기록 — 기억 저장의 단위
-- ════════════════════════════════════════════════════════════════

/-- 커밋 시 저장되는 메타데이터 -/
structure CommitRecord where
  commitHash     : String
  date           : DateStr
  message        : String
  specLinks      : List SpecCodeLink   := []
  trials         : List TrialRecord    := []
  contradictions : List Contradiction  := []
  deriving Repr

-- ════════════════════════════════════════════════════════════════
-- §8. 상태 전이 시스템 (v3에서 유지)
-- ════════════════════════════════════════════════════════════════

/-- 상태 전이 시스템. buggy→fixed 모델링용. -/
class Sys (S : Type) (A : Type) where
  step : S → A → S

def Sys.preserves (S A : Type) [Sys S A] (inv : S → Prop) : Prop :=
  ∀ (s : S) (a : A), inv s → inv (Sys.step s a)

-- ════════════════════════════════════════════════════════════════
-- §9. 불변식 — 4대 원칙의 컴파일타임 강제
-- ════════════════════════════════════════════════════════════════

/-- 분산성: spec과 code 둘 다 존재해야 유효 -/
def SpecCodeLink.isValid (link : SpecCodeLink) : Bool :=
  link.spec.length > 0 && link.code.length > 0

/-- 성숙성: stability ≥ 70인 기억만 승격 가능 -/
def canPromote (sem : SemanticMemory) : Bool :=
  sem.stability.val ≥ 70 && sem.maturity == .consolidated

/-- 모순 대처: 미해소 모순이 있으면 true -/
def hasUnresolved (cs : List Contradiction) : Bool :=
  cs.any (fun c => c.status == .detected || c.status == .investigating)

-- ════════════════════════════════════════════════════════════════
-- §10. Task 불변식 (v3)
-- ════════════════════════════════════════════════════════════════

inductive ValidTaskTransition : TaskStatus → TaskStatus → Prop where
  | pendingToInProgress : ValidTaskTransition .pending .inProgress
  | inProgressToCompleted : ValidTaskTransition .inProgress .completed
  | inProgressToBlocked : ValidTaskTransition .inProgress .blocked
  | blockedToInProgress : ValidTaskTransition .blocked .inProgress

theorem completedIsFinal : ¬ ∃ s, ValidTaskTransition .completed s := by
  intro ⟨_, h⟩; cases h

-- ════════════════════════════════════════════════════════════════
-- §11. Helpers
-- ════════════════════════════════════════════════════════════════

def Context.addTag (t : String) (ctx : Context) : Context :=
  { ctx with tags := t :: ctx.tags }

def Context.addTech (t : String) (ctx : Context) : Context :=
  { ctx with technology := t :: ctx.technology }

def EpisodicMemory.isHighImportance (ep : EpisodicMemory) : Bool :=
  match ep.emotionalValence with
  | some v => v ≥ 7
  | none   => false

def SemanticMemory.isHighConfidence (sem : SemanticMemory) : Bool :=
  sem.stability.val ≥ 80

def MemoryDay.isValid (d : MemoryDay) : Bool :=
  !d.sessionFiles.isEmpty && !d.date.isEmpty

end LongTermMemory.MemorySchema
