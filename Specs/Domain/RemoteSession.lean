/-
  Domain/RemoteSession.lean -- Remote CCR Session Types
  Background remote agents, teleport, polling.
-/
import Common.Types

namespace ClaudeCode.RemoteSession

open ClaudeCode

-- Remote Task Type
inductive RemoteTaskType where
  | standard
  | autofixPr
  | codeReview
  | scheduled
  deriving Repr, BEq

-- Remote Agent Precondition
inductive RemoteAgentPreconditionResult where
  | ready
  | needsAuth (message : String)
  | needsSetup (message : String)
  | unavailable (reason : String)
  deriving Repr

-- Remote Task Metadata
structure AutofixPrMeta where
  prNumber   : Nat
  prUrl      : String
  repository : String
  deriving Repr

-- Remote Agent Task State
structure RemoteAgentTaskState where
  taskId        : String
  remoteType    : RemoteTaskType
  sessionUrl    : Option String := none
  pollIntervalMs : Nat := 5000
  deriving Repr

-- Background Remote Session
structure BackgroundRemoteSession where
  sessionId      : String
  taskId         : String
  status         : String
  createdAt      : Timestamp
  deriving Repr

-- Session Context Source
inductive SessionContextSource where
  | git (repo : String) (branch : String)
  | knowledgeBase (id : String)
  deriving Repr

-- Session Context
structure SessionContext where
  sources   : List SessionContextSource
  resources : List String := []
  deriving Repr

-- Poll Remote Session Response
structure PollRemoteSessionResponse where
  status       : String
  isTerminal   : Bool
  outputUrl    : Option String := none
  errorMessage : Option String := none
  deriving Repr

-- Code Session (full API response)
structure CodeSession where
  sessionId   : String
  status      : String
  createdAt   : Timestamp
  updatedAt   : Option Timestamp := none
  title       : Option String := none
  context     : Option SessionContext := none
  deriving Repr

-- Remote Trigger Config
structure RemoteTriggerConfig where
  schedule    : String       -- cron expression
  prompt      : String
  model       : Option String := none
  maxBudget   : Option Nat := none
  deriving Repr

def remoteSessionBoundaryMap : List BoundaryEntry := [
  { fnName := "RemoteAgentTaskState",     traits := [.io, .network], target := .typescript },
  { fnName := "PollRemoteSessionResponse", traits := [.io, .network], target := .typescript },
  { fnName := "RemoteTriggerConfig",      traits := [.io, .network], target := .typescript },
  { fnName := "RemoteAgentPreconditionResult", traits := [.pure],    target := .typescript }
]

end ClaudeCode.RemoteSession
