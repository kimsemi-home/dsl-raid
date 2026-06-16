use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type DiagnosticRef = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropositionResult {
    pub id: String,
    pub layer: String,
    pub status: String,
    pub severity: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    #[serde(default)]
    pub diagnostics: Vec<DiagnosticRef>,
    #[serde(default)]
    pub assertions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertionResult {
    pub id: String,
    pub proposition: String,
    pub code: String,
    pub predicate: String,
    pub status: String,
    pub severity: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    #[serde(default)]
    pub evidence: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
    #[serde(default)]
    pub diagnostics: Vec<DiagnosticRef>,
}
