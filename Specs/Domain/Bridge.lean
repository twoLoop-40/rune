/-
  Domain/Bridge.lean — 리모트 컨트롤 브릿지
  외부에서 Claude Code 세션을 제어하는 프로토콜.
-/
import Common.Types

namespace ClaudeCode.Bridge

open ClaudeCode

-- ═══════════════════════════════════════════════
-- Bridge Connection
-- ═══════════════════════════════════════════════

/-- 브릿지 연결 상태. -/
inductive BridgeConnectionState where
  | disconnected
  | connecting
  | connected (sessionUrl : String)
  | error (msg : String)
  deriving Repr, BEq

/-- 브릿지 설정. -/
structure BridgeConfig where
  enabled    : Bool := false
  explicit   : Bool := false    -- 사용자가 명시적으로 켰는지
  sessionUrl : Option String := none
  deriving Repr

-- ═══════════════════════════════════════════════
-- Bridge Protocol Messages
-- ═══════════════════════════════════════════════

/-- 브릿지 메시지 유형. -/
inductive BridgeMessageType where
  | prompt           -- 프롬프트 전송
  | response         -- 응답 수신
  | statusQuery      -- 상태 조회
  | statusResponse   -- 상태 응답
  | interrupt        -- 인터럽트
  | heartbeat        -- 연결 유지
  deriving Repr, BEq

/-- 브릿지 메시지. -/
structure BridgeMessage where
  type      : BridgeMessageType
  payload   : JsonValue
  timestamp : Timestamp
  deriving Repr

-- ═══════════════════════════════════════════════
-- BoundaryMap
-- ═══════════════════════════════════════════════

def bridgeBoundaryMap : List BoundaryEntry := [
  { fnName := "BridgeConnectionState", traits := [.io, .network], target := .typescript },
  { fnName := "BridgeMessage",         traits := [.io, .network], target := .typescript }
]

end ClaudeCode.Bridge
