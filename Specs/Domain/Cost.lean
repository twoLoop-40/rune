/-
  Domain/Cost.lean -- Cost Tracking
  Token usage accounting, budget limits, rate limiting.
-/
import Common.Types

namespace ClaudeCode.Cost

open ClaudeCode

-- Cost Entry
structure CostEntry where
  model          : String
  inputTokens    : Nat
  outputTokens   : Nat
  cacheCreation  : Nat := 0
  cacheRead      : Nat := 0
  costMicroUsd   : Nat
  timestamp      : Timestamp
  deriving Repr

-- Budget State
structure BudgetState where
  maxBudgetUsd   : Option Nat := none  -- microdollars
  spentUsd       : Nat := 0            -- microdollars
  entries        : List CostEntry := []
  deriving Repr

-- Budget Check Result
inductive BudgetCheckResult where
  | withinBudget (remaining : Nat)
  | exceeded (overage : Nat)
  | noBudget
  deriving Repr

-- Rate Limit Info
structure RateLimitInfo where
  retryAfterMs   : Nat
  limitType      : String  -- "tokens" | "requests"
  message        : String
  deriving Repr

-- Rate Limit Message
structure RateLimitMessage where
  model      : String
  info       : RateLimitInfo
  timestamp  : Timestamp
  deriving Repr

-- Budget exceeded check
def BudgetState.isExceeded (b : BudgetState) : Bool :=
  match b.maxBudgetUsd with
  | none => false
  | some max => b.spentUsd > max

def costBoundaryMap : List BoundaryEntry := [
  { fnName := "BudgetState",        traits := [.pure],     target := .typescript },
  { fnName := "BudgetCheckResult",  traits := [.pure],     target := .typescript },
  { fnName := "RateLimitInfo",      traits := [.io, .network], target := .typescript },
  { fnName := "CostEntry",          traits := [.pure],     target := .typescript }
]

end ClaudeCode.Cost
