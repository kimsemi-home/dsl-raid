use crate::builder::AssertionSpec;
use crate::{AssertionResult, DiagnosticRef};

pub(crate) fn assertion(spec: AssertionSpec, diagnostics: Vec<DiagnosticRef>) -> AssertionResult {
    AssertionResult {
        id: spec.assertion.to_string(),
        proposition: spec.proposition.to_string(),
        code: spec.code.to_string(),
        predicate: spec.predicate.to_string(),
        status: spec.status.to_string(),
        severity: spec.severity.to_string(),
        subjects: spec.subjects,
        evidence: spec.evidence,
        message: spec.message,
        suggestion: spec.suggestion,
        diagnostics,
    }
}

pub(crate) fn is_blocking(assertion: &AssertionResult) -> bool {
    assertion.status == "failed" && assertion.severity == "error"
}
