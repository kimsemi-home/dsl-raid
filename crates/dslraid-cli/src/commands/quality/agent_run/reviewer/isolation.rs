use crate::commands::quality::agent_run::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let Some(producer) = text(value, &["producer", "id"]) else {
        return;
    };
    for reviewer in items(value, "reviewers") {
        if field_text(reviewer, "id") == Some(producer) {
            issues.push(format!("producer {producer} cannot be listed as reviewer"));
        }
    }
}
