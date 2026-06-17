use super::fields::{field_is, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if !has_high_quality_evidence(value) {
        issues.push("approved run requires high quality evidence".to_string());
    }
    if !has_trace_evidence(value) {
        issues.push("approved run requires trace evidence".to_string());
    }
    if !has_coverage_evidence(value) {
        issues.push("approved run requires coverage evidence".to_string());
    }
}

fn has_high_quality_evidence(value: &Value) -> bool {
    items(value, "evidence").any(|item| field_is(item, "quality", "high"))
}

fn has_trace_evidence(value: &Value) -> bool {
    items(value, "evidence").any(|item| field_is(item, "kind", "trace"))
}

fn has_coverage_evidence(value: &Value) -> bool {
    items(value, "evidence").any(|item| field_is(item, "kind", "coverage"))
}
