use super::required;
use crate::commands::quality::agent_run::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if required::aborted_signal(value) && approved(value) {
        issues.push("aborted run cannot have approved authority gate".to_string());
    }
}

fn approved(value: &Value) -> bool {
    text(value, &["authority_gate", "decision"]) == Some("approved")
}
