/-
  Domain/TextInput.lean -- Text Input Types
  Input modes, ghost text, prompt state.
-/
import Common.Types

namespace ClaudeCode.TextInput

open ClaudeCode

-- Prompt Input Mode
inductive PromptInputMode where
  | bash
  | prompt
  | orphanedPermission
  | taskNotification
  deriving Repr, BEq

-- Editable subset
inductive EditablePromptInputMode where
  | bash
  | prompt
  deriving Repr, BEq

-- Inline Ghost Text (autocomplete)
structure InlineGhostText where
  text       : String
  startCol   : Nat
  deriving Repr

-- Orphaned Permission
structure OrphanedPermission where
  toolName   : String
  inputSummary : String
  agentId    : AgentId
  deriving Repr

-- Base Input State
structure BaseInputState where
  buffer     : String := ""
  cursorPos  : Nat := 0
  ghostText  : Option InlineGhostText := none
  deriving Repr

-- Text Input State
structure TextInputState extends BaseInputState where
  mode       : EditablePromptInputMode := .prompt
  deriving Repr

def textInputBoundaryMap : List BoundaryEntry := [
  { fnName := "PromptInputMode",    traits := [.pure, .ui], target := .typescript },
  { fnName := "TextInputState",     traits := [.pure, .ui], target := .typescript },
  { fnName := "OrphanedPermission", traits := [.pure],      target := .typescript }
]

end ClaudeCode.TextInput
