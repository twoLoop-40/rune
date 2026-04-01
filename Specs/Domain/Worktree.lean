/-
  Domain/Worktree.lean -- Git Worktree Session
  Isolated filesystem scope for branching work.
-/
import Common.Types

namespace ClaudeCode.Worktree

open ClaudeCode

-- Worktree Session
structure WorktreeSession where
  originalCwd     : FilePath
  worktreePath    : FilePath
  worktreeName    : String
  worktreeBranch  : String
  sessionId       : SessionId
  tmuxSessionName : Option String := none
  creationDurMs   : Option Nat := none
  usedSparsePaths : List String := []
  deriving Repr

-- Spawn Mode
inductive SpawnMode where
  | singleSession
  | worktree
  | sameDir
  deriving Repr, BEq

-- Persisted Worktree Session (resume)
structure PersistedWorktreeSession where
  session     : WorktreeSession
  taskId      : Option String := none
  isActive    : Bool := true
  deriving Repr

def worktreeBoundaryMap : List BoundaryEntry := [
  { fnName := "WorktreeSession",  traits := [.io, .systems], target := .typescript },
  { fnName := "SpawnMode",        traits := [.pure],          target := .typescript }
]

end ClaudeCode.Worktree
