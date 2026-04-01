/-
  Domain/Model.lean -- Model Configuration & Pricing
  API providers, model aliases, capabilities, cost tracking.
-/
import Common.Types

namespace ClaudeCode.Model

open ClaudeCode

-- API Provider
inductive APIProvider where
  | anthropic
  | bedrock
  | vertex
  | custom (endpoint : String)
  deriving Repr, BEq

-- Model Alias
inductive ModelAlias where
  | sonnet
  | opus
  | haiku
  deriving Repr, BEq

-- Model Capability
inductive ModelCapability where
  | thinking
  | vision
  | toolUse
  | streaming
  | extendedOutput
  | computerUse
  deriving Repr, BEq

-- Canonical Model ID
structure CanonicalModelId where
  val : String
  deriving Repr, BEq, Hashable

-- Model Config
structure ModelConfig where
  provider       : APIProvider
  modelId        : CanonicalModelId
  alias          : Option ModelAlias := none
  capabilities   : List ModelCapability := []
  maxOutputTokens : Nat := 16384
  contextWindow  : Nat := 200000
  deriving Repr

-- Model Costs (per million tokens, in microdollars)
structure ModelCosts where
  inputTokens    : Nat
  outputTokens   : Nat
  cacheCreation  : Nat := 0
  cacheRead      : Nat := 0
  webSearch      : Nat := 0
  deriving Repr

-- Model Strings (human-readable)
structure ModelStrings where
  displayName : String
  shortName   : String
  description : String := ""
  deriving Repr

-- Model Override Config
structure ModelOverrideConfig where
  modelId     : String
  reason      : String
  deriving Repr

-- Model Option (for model selector UI)
structure ModelOption where
  modelId     : CanonicalModelId
  displayName : String
  isDefault   : Bool := false
  isAvailable : Bool := true
  deriving Repr

-- Bedrock Region
structure BedrockRegionPrefix where
  region : String
  regionPrefix : String
  deriving Repr

-- Cost Tracker State
structure CostTrackerState where
  totalInputTokens   : Nat := 0
  totalOutputTokens  : Nat := 0
  totalCacheCreation : Nat := 0
  totalCacheRead     : Nat := 0
  totalCostUsd       : Nat := 0  -- microdollars
  turnCount          : Nat := 0
  deriving Repr

-- API Metrics Entry
structure ApiMetricsEntry where
  ttftMs     : Nat
  model      : String
  timestamp  : Timestamp
  deriving Repr

def modelBoundaryMap : List BoundaryEntry := [
  { fnName := "ModelConfig",       traits := [.pure],     target := .typescript },
  { fnName := "ModelCosts",        traits := [.pure],     target := .typescript },
  { fnName := "CostTrackerState",  traits := [.pure],     target := .typescript },
  { fnName := "APIProvider",       traits := [.pure, .network], target := .typescript }
]

end ClaudeCode.Model
