// @spec Specs/Domain/Cost.lean ClaudeCode.Cost

use serde::{Deserialize, Serialize};

use crate::types::Timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEntry {
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    #[serde(default)]
    pub cache_creation: u64,
    #[serde(default)]
    pub cache_read: u64,
    pub cost_micro_usd: u64,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BudgetState {
    pub max_budget_usd: Option<u64>,
    #[serde(default)]
    pub spent_usd: u64,
    #[serde(default)]
    pub entries: Vec<CostEntry>,
}

#[derive(Debug, Clone)]
pub enum BudgetCheckResult {
    WithinBudget { remaining: u64 },
    Exceeded { overage: u64 },
    NoBudget,
}

impl BudgetState {
    /// Lean: `def BudgetState.isExceeded`
    pub fn is_exceeded(&self) -> bool {
        match self.max_budget_usd {
            None => false,
            Some(max) => self.spent_usd > max,
        }
    }

    pub fn check(&self) -> BudgetCheckResult {
        match self.max_budget_usd {
            None => BudgetCheckResult::NoBudget,
            Some(max) if self.spent_usd > max => BudgetCheckResult::Exceeded {
                overage: self.spent_usd - max,
            },
            Some(max) => BudgetCheckResult::WithinBudget {
                remaining: max - self.spent_usd,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub retry_after_ms: u64,
    pub limit_type: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitMessage {
    pub model: String,
    pub info: RateLimitInfo,
    pub timestamp: Timestamp,
}
