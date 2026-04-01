/-
  Domain/Compaction.lean -- Context Compaction & Management
  Message compaction strategies, auto-compact, prompt cache.
-/
import Common.Types

namespace ClaudeCode.Compaction

open ClaudeCode

-- Compaction Strategy
inductive CompactionStrategy where
  | summarize
  | truncate
  | collapse
  deriving Repr, BEq

-- Time-based MC Config
structure TimeBasedMCConfig where
  intervalMs     : Nat := 300000
  maxMessages    : Nat := 100
  deriving Repr

-- Auto-Compact Tracking
structure AutoCompactTrackingState where
  lastCompactTime   : Option Timestamp := none
  messagesSince     : Nat := 0
  compactionCount   : Nat := 0
  deriving Repr

-- Recompaction Info
structure RecompactionInfo where
  originalSize  : Nat
  compactedSize : Nat
  removedCount  : Nat
  deriving Repr

-- Context Edit Strategy
inductive ContextEditStrategy where
  | replace
  | append
  | prepend
  deriving Repr, BEq

-- Context Management Config
structure ContextManagementConfig where
  maxContextTokens  : Nat := 200000
  compactThreshold  : Float := 0.8
  strategy          : CompactionStrategy := .summarize
  deriving Repr

-- Pending Cache Edits
structure PendingCacheEdits where
  edits      : List (String × String)  -- (key, newValue)
  timestamp  : Timestamp
  deriving Repr

-- Microcompact Result
structure MicrocompactResult where
  removedTokens  : Nat
  removedMessages : Nat
  strategy       : CompactionStrategy
  deriving Repr

-- Context Collapse Commit Entry
structure ContextCollapseCommitEntry where
  collapseId : String
  summary    : String
  deriving Repr

-- Context Collapse Snapshot Entry
structure ContextCollapseSnapshotEntry where
  staged : List String
  armed  : List String
  deriving Repr

def compactionBoundaryMap : List BoundaryEntry := [
  { fnName := "CompactionStrategy",       traits := [.pure],     target := .typescript },
  { fnName := "ContextManagementConfig",  traits := [.pure],     target := .typescript },
  { fnName := "AutoCompactTrackingState", traits := [.pure],     target := .typescript },
  { fnName := "MicrocompactResult",       traits := [.pure],     target := .typescript }
]

end ClaudeCode.Compaction
