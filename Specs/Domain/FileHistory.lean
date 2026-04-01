/-
  Domain/FileHistory.lean -- File Versioning & Attribution
  Backup tracking, diff stats, character-level attribution.
-/
import Common.Types

namespace ClaudeCode.FileHistory

open ClaudeCode

-- File History Backup
structure FileHistoryBackup where
  filePath    : FilePath
  backupPath  : FilePath
  timestamp   : Timestamp
  toolUseId   : String
  agentId     : Option AgentId := none
  deriving Repr

-- File History Snapshot
structure FileHistorySnapshot where
  messageId : String
  backups   : List FileHistoryBackup
  deriving Repr

-- File History State
structure FileHistoryState where
  files       : List FileHistoryBackup := []
  snapshots   : List FileHistorySnapshot := []
  deriving Repr

-- Diff Stats
structure DiffStats where
  changedFiles : Nat
  insertions   : Nat
  deletions    : Nat
  deriving Repr

-- File Attribution State (per-file, character-level)
structure FileAttributionState where
  filePath     : FilePath
  totalChars   : Nat
  claudeChars  : Nat
  humanChars   : Nat
  deriving Repr

-- Attribution Snapshot
structure AttributionSnapshot where
  surface    : String
  fileStates : List FileAttributionState
  deriving Repr

-- Attribution State (root)
structure AttributionState where
  files     : List FileAttributionState := []
  snapshots : List AttributionSnapshot := []
  deriving Repr

-- File State Cache
structure FileStateCache where
  entries    : List (FilePath × String)  -- path -> content hash
  maxEntries : Nat := 1000
  deriving Repr

def fileHistoryBoundaryMap : List BoundaryEntry := [
  { fnName := "FileHistoryState",    traits := [.io],       target := .typescript },
  { fnName := "DiffStats",          traits := [.pure],     target := .typescript },
  { fnName := "FileAttributionState", traits := [.pure],   target := .typescript },
  { fnName := "FileStateCache",     traits := [.io],       target := .typescript }
]

end ClaudeCode.FileHistory
