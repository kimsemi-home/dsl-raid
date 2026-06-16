use serde::{Deserialize, Serialize};

use super::{AssertionResult, DiagnosticRef, PropositionResult, Summary};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub validation_version: String,
    pub source: ValidationSource,
    pub run: ValidationRun,
    pub summary: Summary,
    pub propositions: Vec<PropositionResult>,
    pub assertions: Vec<AssertionResult>,
    #[serde(default)]
    pub diagnostics: Vec<DiagnosticRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSource {
    pub core_ir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ir_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRun {
    pub tool: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub mode: String,
    #[serde(default)]
    pub deny: Vec<String>,
}

impl ValidationReport {
    pub fn has_blocking_errors(&self) -> bool {
        self.assertions
            .iter()
            .any(|assertion| assertion.status == "failed" && assertion.severity == "error")
    }

    pub fn has_denied_warnings(&self, denied: &[String]) -> bool {
        denied.iter().any(|severity| severity == "warning")
            && self
                .assertions
                .iter()
                .any(|assertion| assertion.status == "warning")
    }

    pub fn is_success(&self, denied: &[String]) -> bool {
        !self.has_blocking_errors() && !self.has_denied_warnings(denied)
    }
}
