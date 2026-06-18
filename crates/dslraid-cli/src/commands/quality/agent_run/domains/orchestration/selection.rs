use crate::commands::quality::agent_run::fields::{field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    let Some(producer) = text(value, &["producer", "id"]) else {
        return;
    };
    if !has_selected_reviewers(item) {
        return;
    }
    if field_text(item, "routed_by") == Some(producer) {
        issues.push(format!(
            "producer {producer} cannot route reviewer selection"
        ));
    }
}

fn has_selected_reviewers(value: &Value) -> bool {
    value
        .get("selected_reviewers")
        .and_then(Value::as_array)
        .is_some_and(|items| !items.is_empty())
}
