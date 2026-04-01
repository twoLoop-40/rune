/-
  Common/Types.lean — Claude Code 공통 타입 정의
  모든 도메인 Spec이 import하는 기초 타입.
-/

namespace ClaudeCode

-- ═══════════════════════════════════════════════
-- Branded ID Types (컴파일타임 안전성)
-- ═══════════════════════════════════════════════

/-- 세션 식별자. getSessionId()로 생성. -/
structure SessionId where
  val : String
  deriving Repr, BEq, Hashable

/-- 에이전트 식별자. 패턴: /^a(?:.+-)?[0-9a-f]{16}$/ -/
structure AgentId where
  val : String
  deriving Repr, BEq, Hashable

/-- UUID 문자열 래퍼. -/
structure UUID where
  val : String
  deriving Repr, BEq, Hashable

-- ═══════════════════════════════════════════════
-- Timestamp & Duration
-- ═══════════════════════════════════════════════

/-- 밀리초 단위 타임스탬프. -/
structure Timestamp where
  ms : Nat
  deriving Repr, BEq, Ord

/-- 밀리초 단위 지속시간. -/
structure Duration where
  ms : Nat
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Result / Option wrappers
-- ═══════════════════════════════════════════════

/-- 도메인 에러. -/
inductive DomainError where
  | validation (msg : String)
  | permission (msg : String)
  | timeout (msg : String)
  | notFound (msg : String)
  | internal (msg : String)
  deriving Repr

/-- Lean 표준 Except 활용. -/
abbrev DomainResult (α : Type) := Except DomainError α

-- ═══════════════════════════════════════════════
-- File System Primitives
-- ═══════════════════════════════════════════════

/-- 절대 파일 경로. -/
structure FilePath where
  val : String
  deriving Repr, BEq, Hashable

/-- Glob 패턴. -/
structure GlobPattern where
  val : String
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- JSON-like Value (스키마 표현용)
-- ═══════════════════════════════════════════════

inductive JsonValue where
  | null
  | bool (b : Bool)
  | num (n : Int)
  | str (s : String)
  | arr (xs : List JsonValue)
  | obj (kvs : List (String × JsonValue))
  deriving Repr

-- ═══════════════════════════════════════════════
-- Source / Origin 공통
-- ═══════════════════════════════════════════════

/-- 설정 소스. -/
inductive SettingSource where
  | userSettings
  | projectSettings
  | envVar
  | cliFlag
  | pluginConfig
  deriving Repr, BEq

/-- 플러그인 소스. -/
structure PluginSource where
  name : String
  repository : String
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Content Block (API 프로토콜)
-- ═══════════════════════════════════════════════

/-- Anthropic API ContentBlockParam 추상화. -/
inductive ContentBlock where
  | text (content : String)
  | image (mediaType : String) (data : String)
  | toolUse (id : String) (name : String) (input : JsonValue)
  | toolResult (toolUseId : String) (content : String) (isError : Bool)
  | thinking (content : String)
  deriving Repr

-- ═══════════════════════════════════════════════
-- Model & Effort
-- ═══════════════════════════════════════════════

/-- 모델 설정. -/
inductive ModelSetting where
  | default
  | named (modelId : String)
  deriving Repr, BEq

/-- 추론 노력 수준. -/
inductive EffortValue where
  | low
  | medium
  | high
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Boundary Annotation (gdd-unified용)
-- ═══════════════════════════════════════════════

/-- 함수 특성 태그 (경계 분류용). -/
inductive FnTrait where
  | pure
  | provable
  | io
  | concurrent
  | network
  | cli
  | ui
  | reactive
  | ml
  | datascience
  | discovery
  | systems
  deriving Repr, BEq

/-- 코드 생성 타겟 언어. -/
inductive TargetLang where
  | c
  | rust
  | go
  | python
  | typescript
  deriving Repr, BEq

/-- 경계 분류 엔트리. -/
structure BoundaryEntry where
  fnName : String
  traits : List FnTrait
  target : TargetLang
  deriving Repr

end ClaudeCode
