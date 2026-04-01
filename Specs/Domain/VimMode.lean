/-
  Domain/VimMode.lean -- Vim Input Mode
  Vim emulation state machine for CLI text input.
-/
import Common.Types

namespace ClaudeCode.VimMode

open ClaudeCode

-- Vim Mode
inductive VimMode where
  | insert
  | normal
  deriving Repr, BEq

-- Vim Operator
inductive Operator where
  | delete
  | change
  | yank
  deriving Repr, BEq

-- Find Type
inductive FindType where
  | find        -- f
  | findBack    -- F
  | till        -- t
  | tillBack    -- T
  deriving Repr, BEq

-- Text Object Scope
inductive TextObjScope where
  | inner    -- i
  | around   -- a
  deriving Repr, BEq

-- Vim Command State
structure CommandState where
  operator     : Option Operator := none
  count        : Option Nat      := none
  findType     : Option FindType := none
  findChar     : Option Char     := none
  textObjScope : Option TextObjScope := none
  deriving Repr

-- Persistent State (across commands)
structure PersistentState where
  lastFind     : Option (FindType × Char) := none
  registers    : List (Char × String)     := []
  deriving Repr

-- Vim State (full)
structure VimState where
  mode       : VimMode := .normal
  command    : CommandState := {}
  persistent : PersistentState := {}
  cursorPos  : Nat := 0
  deriving Repr

-- Recorded Change (for dot-repeat)
structure RecordedChange where
  keys   : String
  result : String
  deriving Repr

-- Vim Input State
structure VimInputState where
  mode    : VimMode
  vim     : VimState
  buffer  : String := ""
  cursor  : Nat := 0
  deriving Repr

-- Vim Transitions
inductive VimTransition where
  | insertToNormal
  | normalToInsert
  | stayInNormal
  | stayInInsert
  deriving Repr, BEq

def vimBoundaryMap : List BoundaryEntry := [
  { fnName := "VimState",       traits := [.pure, .provable], target := .typescript },
  { fnName := "VimMode",        traits := [.pure],            target := .typescript },
  { fnName := "VimTransition",  traits := [.pure, .provable], target := .typescript }
]

end ClaudeCode.VimMode
