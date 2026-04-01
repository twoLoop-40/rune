/-
  Domain/Voice.lean -- Voice Recording State
  Audio input, transcription, interim results.
-/
import Common.Types

namespace ClaudeCode.Voice

open ClaudeCode

-- Voice Recording State
inductive VoiceStatus where
  | idle
  | recording
  | processing
  deriving Repr, BEq

-- Voice State
structure VoiceState where
  status           : VoiceStatus := .idle
  error            : Option String := none
  interimTranscript : Option String := none
  audioLevels      : List Nat := []     -- dB values
  warmingUp        : Bool := false
  deriving Repr

def voiceBoundaryMap : List BoundaryEntry := [
  { fnName := "VoiceState",  traits := [.io, .ui], target := .typescript }
]

end ClaudeCode.Voice
