/-
  Domain/Session.lean -- Session Management
  Session lifecycle, activity tracking, storage, resume.
-/
import Common.Types

namespace ClaudeCode.Session

open ClaudeCode

-- Session State
inductive SessionState where
  | idle
  | running
  | requiresAction
  deriving Repr, BEq

-- Session Activity Type
inductive SessionActivityType where
  | toolStart
  | text
  | result
  | error
  deriving Repr, BEq

-- Session Activity Reason
inductive SessionActivityReason where
  | apiCall
  | toolExec
  deriving Repr, BEq

-- Session Activity
structure SessionActivity where
  activityType : SessionActivityType
  reason       : SessionActivityReason
  timestamp    : Timestamp
  description  : Option String := none
  deriving Repr

-- Session Done Status
inductive SessionDoneStatus where
  | completed
  | failed
  | interrupted
  deriving Repr, BEq

-- Parsed Session URL
structure ParsedSessionUrl where
  host      : String
  sessionId : String
  token     : Option String := none
  deriving Repr

-- Session Info (listing)
structure SessionInfo where
  sessionId   : SessionId
  title       : Option String := none
  lastPrompt  : Option String := none
  timestamp   : Timestamp
  cwd         : FilePath
  tags        : List String := []
  deriving Repr

-- Lite Session File
structure LiteSessionFile where
  path      : FilePath
  sessionId : SessionId
  modTime   : Timestamp
  deriving Repr

-- Session External Metadata
structure SessionExternalMeta where
  sessionId : SessionId
  prLinks   : List String := []
  tags      : List String := []
  deriving Repr

-- Session Storage Config
structure SessionStorageConfig where
  storagePath  : FilePath
  maxSessions  : Nat := 1000
  deriving Repr

-- Session Memory Config
structure SessionMemoryConfig where
  enabled       : Bool := true
  memoryScope   : String := "personal"
  deriving Repr

def sessionBoundaryMap : List BoundaryEntry := [
  { fnName := "SessionState",       traits := [.pure],     target := .typescript },
  { fnName := "SessionActivity",    traits := [.pure, .io], target := .typescript },
  { fnName := "SessionInfo",        traits := [.pure],     target := .typescript },
  { fnName := "SessionStorageConfig", traits := [.io],     target := .typescript }
]

end ClaudeCode.Session
