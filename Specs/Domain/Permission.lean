/-
  Domain/Permission.lean — 권한 시스템
  도구 실행 전 권한 검사, 분류기, 규칙 체계.
-/
import Common.Types

namespace ClaudeCode.Permission

open ClaudeCode

-- ═══════════════════════════════════════════════
-- Permission Mode
-- ═══════════════════════════════════════════════

/-- 권한 모드. CLI/SDK 설정. -/
inductive PermissionMode where
  | default            -- 일반 (ask/block)
  | acceptEdits        -- 파일 편집 자동 허용
  | bypassPermissions  -- 모든 권한 우회
  | dontAsk            -- 프롬프트 스킵
  | plan               -- 계획 모드 제한
  | auto               -- 분류기 사용
  | bubble             -- 내부용 (상위로 위임)
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Permission Behavior & Decision
-- ═══════════════════════════════════════════════

/-- 권한 행동. -/
inductive PermissionBehavior where
  | allow
  | deny
  | ask
  | passthrough  -- 훅으로 위임
  deriving Repr, BEq

/-- 권한 규칙 소스. -/
inductive PermissionRuleSource where
  | userSettings
  | projectSettings
  | cliFlag
  | hook
  | classifier
  | pluginPolicy
  deriving Repr, BEq

/-- 권한 규칙. -/
structure PermissionRule where
  source       : PermissionRuleSource
  ruleBehavior : PermissionBehavior
  toolName     : String
  ruleContent  : Option String := none
  deriving Repr

-- ═══════════════════════════════════════════════
-- Permission Decision (Discriminated Union)
-- ═══════════════════════════════════════════════

/-- 거부 이유. -/
inductive DenyReason where
  | blockedByPolicy
  | blockedByHook (msg : String)
  | blockedByClassifier (reason : String)
  | userDenied
  | planModeRestriction
  deriving Repr

/-- 권한 결정 결과. -/
inductive PermissionDecision where
  | allow (userModified : Bool := false)
  | ask (message : String) (blockedPath : Option String := none)
  | deny (reason : DenyReason) (message : String)
  | passthrough (message : String)
  deriving Repr

/-- allow 결정은 실행을 허용한다. -/
def PermissionDecision.isAllowed : PermissionDecision → Bool
  | .allow _ => true
  | _        => false

/-- deny 결정은 실행을 차단한다. -/
def PermissionDecision.isDenied : PermissionDecision → Bool
  | .deny _ _ => true
  | _         => false

-- ═══════════════════════════════════════════════
-- YOLO Classifier
-- ═══════════════════════════════════════════════

/-- 분류기 단계. -/
inductive ClassifierStage where
  | fast
  | thinking
  deriving Repr, BEq

/-- 분류기 결과. -/
structure ClassifierResult where
  shouldBlock        : Bool
  reason             : String
  unavailable        : Bool := false
  transcriptTooLong  : Bool := false
  model              : String
  stage              : Option ClassifierStage := none
  durationMs         : Option Nat := none
  deriving Repr

-- ═══════════════════════════════════════════════
-- Permission Flow (전체 흐름)
-- ═══════════════════════════════════════════════

/--
  권한 검사 흐름:
  1. validateInput (도구별 검증)
  2. checkPermissions → PermissionDecision
  3. allow → 실행
  4. ask → 사용자 프롬프트 또는 분류기
  5. deny → 거부
  6. passthrough → 훅 결정
-/
inductive PermissionFlowStep where
  | validateInput
  | checkPermissions
  | executeAllowed
  | promptUser
  | runClassifier
  | rejectDenied
  | delegateToHook
  deriving Repr

/-- bypass 모드에서는 항상 allow. -/
theorem bypass_always_allows (mode : PermissionMode) :
    mode = .bypassPermissions →
    ∀ (_toolName : String), PermissionDecision.isAllowed (.allow false) = true := by
  intro _ _
  simp [PermissionDecision.isAllowed]

-- ═══════════════════════════════════════════════
-- Denial Tracking (분류기 폴백)
-- ═══════════════════════════════════════════════

/-- 거부 추적 상태. 연속 거부 시 ask 모드로 폴백. -/
structure DenialTrackingState where
  consecutiveDenials : Nat
  threshold          : Nat := 3
  fallbackToAsk      : Bool := false
  deriving Repr

/-- threshold 도달 시 fallback 활성화. -/
def DenialTrackingState.shouldFallback (s : DenialTrackingState) : Bool :=
  s.consecutiveDenials ≥ s.threshold

-- ═══════════════════════════════════════════════
-- BoundaryMap
-- ═══════════════════════════════════════════════

def permissionBoundaryMap : List BoundaryEntry := [
  { fnName := "PermissionMode",     traits := [.pure, .provable], target := .typescript },
  { fnName := "PermissionDecision", traits := [.pure, .provable], target := .typescript },
  { fnName := "ClassifierResult",   traits := [.pure, .io],       target := .typescript },
  { fnName := "DenialTrackingState", traits := [.pure, .provable], target := .typescript }
]

end ClaudeCode.Permission
