use crate::commands::quality::agent_run::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if text(value, &["authority_gate", "decision"]) != Some("escalated") {
        return;
    }
    if !human_review_required(value) {
        return;
    }
    let approver = text(value, &["authority_gate", "approved_by"]).unwrap_or("");
    if is_human_or_steward(approver) {
        return;
    }
    issues.push("escalated authority gate requires human or steward target".to_string());
}

fn is_human_or_steward(value: &str) -> bool {
    value.starts_with("human:") || value.starts_with("steward:")
}

fn human_review_required(value: &Value) -> bool {
    value
        .pointer("/authority_gate/human_review_required")
        .and_then(Value::as_bool)
        == Some(true)
}
