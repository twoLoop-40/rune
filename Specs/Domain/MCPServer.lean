/-
  Domain/MCPServer.lean -- MCP & LSP Server Configuration
  Transport types, server configs, elicitation.
-/
import Common.Types

namespace ClaudeCode.MCPServer

open ClaudeCode

-- MCP Config Scope
inductive ConfigScope where
  | user
  | project
  | plugin
  deriving Repr, BEq

-- Transport Type
inductive TransportType where
  | stdio
  | sse
  | webSocket
  | http
  deriving Repr, BEq

-- MCP Stdio Server Config
structure McpStdioConfig where
  command  : String
  args     : List String := []
  env      : List (String × String) := []
  cwd      : Option FilePath := none
  deriving Repr

-- MCP SSE Server Config
structure McpSseConfig where
  url      : String
  headers  : List (String × String) := []
  deriving Repr

-- MCP WebSocket Server Config
structure McpWebSocketConfig where
  url      : String
  headers  : List (String × String) := []
  deriving Repr

-- MCP HTTP Server Config
structure McpHttpConfig where
  url      : String
  headers  : List (String × String) := []
  deriving Repr

-- MCP Server Config (union)
inductive McpServerConfigVariant where
  | stdio (config : McpStdioConfig)
  | sse (config : McpSseConfig)
  | webSocket (config : McpWebSocketConfig)
  | http (config : McpHttpConfig)
  deriving Repr

-- LSP Server Instance
structure LspServerInstance where
  serverName : String
  command    : String
  args       : List String := []
  language   : String
  isRunning  : Bool := false
  deriving Repr

-- Elicitation Waiting State
inductive ElicitationWaitingState where
  | idle
  | waiting (serverName : String)
  | responded
  | timedOut
  deriving Repr, BEq

-- Elicitation Request Event
structure ElicitationRequestEvent where
  serverName  : String
  params      : JsonValue
  requestId   : String
  deriving Repr

-- Channel Permission Response
inductive ChannelPermissionResponse where
  | granted
  | denied (reason : String)
  | pending
  deriving Repr, BEq

-- Server Resource
structure ServerResource where
  uri         : String
  name        : String
  description : Option String := none
  mimeType    : Option String := none
  deriving Repr

def mcpServerBoundaryMap : List BoundaryEntry := [
  { fnName := "McpServerConfigVariant", traits := [.io, .network], target := .typescript },
  { fnName := "LspServerInstance",      traits := [.io],           target := .typescript },
  { fnName := "ElicitationRequestEvent", traits := [.io],          target := .typescript },
  { fnName := "ServerResource",         traits := [.pure, .network], target := .typescript }
]

end ClaudeCode.MCPServer
