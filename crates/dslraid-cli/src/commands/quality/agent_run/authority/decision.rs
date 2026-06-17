use crate::commands::quality::agent_run::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if !non_approved_decision(value) || has_evidence(value) {
        return;
    }
    issues.push("non-approved authority gate requires evidence".to_string());
}

fn non_approved_decision(value: &Value) -> bool {
    matches!(
        text(value, &["authority_gate", "decision"]),
        Some("rejected" | "blocked" | "escalated")
    )
}

fn has_evidence(value: &Value) -> bool {
    value
        .pointer("/authority_gate/evidence")
        .and_then(Value::as_array)
        .is_some_and(|items| !items.is_empty())
}
