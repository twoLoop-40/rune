/-
  Domain/Plugin.lean — 플러그인 시스템
  확장 아키텍처: MCP 서버, 스킬, 훅, LSP.
-/
import Common.Types

namespace ClaudeCode.Plugin

open ClaudeCode

-- ═══════════════════════════════════════════════
-- Plugin Manifest
-- ═══════════════════════════════════════════════

/-- 플러그인 매니페스트. -/
structure PluginManifest where
  name        : String
  version     : String
  description : String
  author      : Option String := none
  repository  : Option String := none
  deriving Repr

-- ═══════════════════════════════════════════════
-- Plugin Definition Types
-- ═══════════════════════════════════════════════

/-- 빌트인 플러그인 정의. -/
structure BuiltinPluginDef where
  name           : String
  description    : String
  version        : Option String := none
  defaultEnabled : Bool := true
  deriving Repr

/-- 로드된 플러그인. -/
structure LoadedPluginDef where
  name         : String
  manifest     : PluginManifest
  path         : FilePath
  source       : String
  repository   : String
  enabled      : Bool := true
  isBuiltin    : Bool := false
  -- 확장 포인트
  hasCommands  : Bool := false
  hasSkills    : Bool := false
  hasHooks     : Bool := false
  hasMcpServers : Bool := false
  hasLspServers : Bool := false
  deriving Repr

-- ═══════════════════════════════════════════════
-- Plugin Error Types (25+ variants)
-- ═══════════════════════════════════════════════

/-- 플러그인 에러 유형. -/
inductive PluginErrorType where
  | pathNotFound
  | gitAuthFailed
  | manifestValidationError
  | pluginNotFound
  | mcpConfigInvalid
  | lspServerCrashed
  | marketplaceBlockedByPolicy
  | dependencyUnsatisfied
  | networkError
  | permissionDenied
  | versionMismatch
  | installFailed
  | loadFailed
  | hookExecutionFailed
  | skillNotFound
  | commandConflict
  | circularDependency
  | sandboxViolation
  | configParseError
  | timeoutExceeded
  | runtimeError
  deriving Repr, BEq

/-- 플러그인 에러. -/
structure PluginError where
  errorType    : PluginErrorType
  source       : String
  message      : String
  pluginName   : Option String := none
  serverName   : Option String := none
  deriving Repr

-- ═══════════════════════════════════════════════
-- MCP Server Config
-- ═══════════════════════════════════════════════

/-- MCP 서버 설정. -/
structure McpServerConfig where
  command  : String
  args     : List String := []
  env      : List (String × String) := []
  cwd      : Option FilePath := none
  deriving Repr

/-- LSP 서버 설정. -/
structure LspServerConfig where
  command  : String
  args     : List String := []
  language : String
  deriving Repr

-- ═══════════════════════════════════════════════
-- Bundled Skill
-- ═══════════════════════════════════════════════

/-- 번들된 스킬 정의. -/
structure BundledSkillDef where
  name        : String
  description : String
  trigger     : List String := []    -- 트리거 키워드
  isHidden    : Bool := false
  deriving Repr

-- ═══════════════════════════════════════════════
-- Plugin Lifecycle
-- ═══════════════════════════════════════════════

/-- 플러그인 라이프사이클. -/
inductive PluginLifecycle where
  | discovered
  | installing
  | installed
  | loading
  | loaded
  | enabled
  | disabled
  | uninstalling
  | uninstalled
  | errored
  deriving Repr, BEq

/-- 유효한 라이프사이클 전이. -/
inductive ValidPluginTransition : PluginLifecycle → PluginLifecycle → Prop where
  | discoverToInstall  : ValidPluginTransition .discovered .installing
  | installToInstalled : ValidPluginTransition .installing .installed
  | installedToLoading : ValidPluginTransition .installed .loading
  | loadingToLoaded    : ValidPluginTransition .loading .loaded
  | loadedToEnabled    : ValidPluginTransition .loaded .enabled
  | enabledToDisabled  : ValidPluginTransition .enabled .disabled
  | disabledToEnabled  : ValidPluginTransition .disabled .enabled
  | anyToError (s : PluginLifecycle) : ValidPluginTransition s .errored

-- ═══════════════════════════════════════════════
-- BoundaryMap
-- ═══════════════════════════════════════════════

def pluginBoundaryMap : List BoundaryEntry := [
  { fnName := "LoadedPluginDef",  traits := [.pure],     target := .typescript },
  { fnName := "PluginError",      traits := [.pure],     target := .typescript },
  { fnName := "McpServerConfig",  traits := [.pure, .io], target := .typescript },
  { fnName := "PluginLifecycle",  traits := [.pure, .provable], target := .typescript }
]

end ClaudeCode.Plugin
