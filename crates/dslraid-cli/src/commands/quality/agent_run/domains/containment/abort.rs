use super::required;
use crate::commands::quality::agent_run::fields::{field_is, field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if required::aborted_signal(value) && approved(value) {
        issues.push("aborted run cannot have approved authority gate".to_string());
    }
    if !required::aborted_signal(value) || !has_abort_bundle(value) {
        return;
    }
    for artifact in items(value, "artifacts").filter(|item| field_is(item, "status", "verified")) {
        issues.push(format!("abort blocks verified artifact {}", id(artifact)));
    }
}

fn approved(value: &Value) -> bool {
    text(value, &["authority_gate", "decision"]) == Some("approved")
}

fn has_abort_bundle(value: &Value) -> bool {
    items(value, "containments").any(|item| field_is(item, "kind", "abort"))
}

fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
