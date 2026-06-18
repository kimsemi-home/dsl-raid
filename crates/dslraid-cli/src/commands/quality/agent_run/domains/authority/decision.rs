use crate::commands::quality::agent_run::fields::text;
use serde_json::Value;

mod escalation;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if non_approved_decision(value) && !has_evidence(value) {
        issues.push("non-approved authority gate requires evidence".to_string());
    }
    if escalated_decision(value) && !human_review_required(value) {
        issues.push("escalated authority gate requires human review".to_string());
    }
    escalation::push_issues(value, issues);
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

fn escalated_decision(value: &Value) -> bool {
    text(value, &["authority_gate", "decision"]) == Some("escalated")
}

fn human_review_required(value: &Value) -> bool {
    value
        .pointer("/authority_gate/human_review_required")
        .and_then(Value::as_bool)
        == Some(true)
}
