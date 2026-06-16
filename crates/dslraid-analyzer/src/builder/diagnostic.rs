use crate::DiagnosticRef;

use super::AssertionSpec;

pub(crate) fn diagnostic_for(spec: &AssertionSpec) -> Vec<DiagnosticRef> {
    if spec.status == "failed" || spec.status == "warning" {
        vec![format!("diagnostic:{}", slug(spec.assertion))]
    } else {
        Vec::new()
    }
}

fn slug(assertion: &str) -> String {
    assertion
        .strip_prefix("assertion:")
        .unwrap_or(assertion)
        .replace('_', "-")
}
