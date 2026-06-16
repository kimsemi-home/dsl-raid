use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub status: String,
    pub propositions: CountSummary,
    pub assertions: CountSummary,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountSummary {
    pub passed: usize,
    pub failed: usize,
    pub warnings: usize,
    pub skipped: usize,
    pub not_applicable: usize,
}
