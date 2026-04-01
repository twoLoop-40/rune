/-
  Domain/State.lean — AppState 루트 상태
  단일 진실 원천(Single Source of Truth). DeepImmutable.
-/
import Common.Types

namespace ClaudeCode.State

open ClaudeCode

-- ═══════════════════════════════════════════════
-- MCP Connection State
-- ═══════════════════════════════════════════════

/-- MCP 서버 연결 상태. -/
inductive McpConnectionStatus where
  | connected
  | connecting
  | disconnected
  | error (msg : String)
  deriving Repr, BEq

/-- MCP 서버 연결. -/
structure McpServerConnection where
  serverName : String
  status     : McpConnectionStatus
  toolCount  : Nat := 0
  deriving Repr

-- ═══════════════════════════════════════════════
-- Plugin State
-- ═══════════════════════════════════════════════

/-- 로드된 플러그인. -/
structure LoadedPlugin where
  name       : String
  path       : FilePath
  source     : String
  repository : String
  enabled    : Bool
  isBuiltin  : Bool := false
  deriving Repr

/-- 플러그인 설치 상태. -/
inductive PluginInstallStatus where
  | installed
  | installing
  | failed (error : String)
  | notInstalled
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- UI State
-- ═══════════════════════════════════════════════

/-- 하단 패널 선택. -/
inductive FooterSelection where
  | tasks
  | tmux
  | bagel
  | teams
  | bridge
  deriving Repr, BEq

/-- 확장 뷰 모드. -/
inductive ExpandedView where
  | none
  | tasks
  | teammates
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Notification & Elicitation
-- ═══════════════════════════════════════════════

/-- 알림. -/
structure Notification where
  message          : String
  notificationType : String
  timestamp        : Timestamp
  deriving Repr

/-- 정보 요청 이벤트. -/
structure ElicitationRequest where
  serverName : String
  params     : JsonValue
  deriving Repr

-- ═══════════════════════════════════════════════
-- Speculation State
-- ═══════════════════════════════════════════════

/-- 투기 실행 상태. -/
structure SpeculationState where
  active    : Bool := false
  accepted  : Nat := 0
  timeSaved : Duration := ⟨0⟩
  deriving Repr

-- ═══════════════════════════════════════════════
-- Team Context (Swarm)
-- ═══════════════════════════════════════════════

/-- 팀원 상태. -/
structure TeammateState where
  agentId    : AgentId
  name       : String
  isLeader   : Bool := false
  deriving Repr

/-- 팀 컨텍스트. -/
structure TeamContext where
  teamName    : String
  leadAgentId : String
  isLeader    : Bool
  teammates   : List TeammateState := []
  deriving Repr

-- ═══════════════════════════════════════════════
-- Inbox (Agent Messages)
-- ═══════════════════════════════════════════════

/-- 수신함 메시지. -/
structure InboxMessage where
  fromAgentId : AgentId
  content     : String
  timestamp   : Timestamp
  deriving Repr

-- ═══════════════════════════════════════════════
-- AppState (Root)
-- ═══════════════════════════════════════════════

/-- 앱 상태. DeepImmutable. 단일 진실 원천. -/
structure AppState where
  -- 설정
  verbose               : Bool := false
  mainLoopModel         : ModelSetting := .default
  isBriefOnly           : Bool := false

  -- MCP
  mcpConnections        : List McpServerConnection := []

  -- 플러그인
  enabledPlugins        : List LoadedPlugin := []
  disabledPlugins       : List LoadedPlugin := []

  -- 태스크
  taskCount             : Nat := 0
  expandedView          : ExpandedView := .none
  foregroundedTaskId    : Option String := none

  -- 팀
  teamContext           : Option TeamContext := none
  inbox                 : List InboxMessage := []

  -- UI
  footerSelection       : Option FooterSelection := none
  spinnerTip            : Option String := none

  -- 투기
  speculation           : SpeculationState := {}

  -- 리모트 브릿지
  replBridgeEnabled     : Bool := false
  replBridgeConnected   : Bool := false
  replBridgeSessionUrl  : Option String := none

  -- 인증 버전
  authVersion           : Nat := 0
  deriving Repr

-- ═══════════════════════════════════════════════
-- State Update Pattern
-- ═══════════════════════════════════════════════

/--
  상태 업데이트 패턴:
  setAppState : (AppState → AppState) → IO Unit
  - 불변 업데이트 (함수형)
  - 구독자에게 변경 알림
  - UI 리렌더링 트리거
-/
structure StateUpdate where
  description : String
  deriving Repr

-- ═══════════════════════════════════════════════
-- BoundaryMap
-- ═══════════════════════════════════════════════

def stateBoundaryMap : List BoundaryEntry := [
  { fnName := "AppState",          traits := [.pure, .provable], target := .typescript },
  { fnName := "McpServerConnection", traits := [.pure, .io],     target := .typescript },
  { fnName := "TeamContext",       traits := [.pure],            target := .typescript },
  { fnName := "SpeculationState",  traits := [.pure],            target := .typescript }
]

end ClaudeCode.State
