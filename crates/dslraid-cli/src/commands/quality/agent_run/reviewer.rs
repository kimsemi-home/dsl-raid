use super::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if independent_reviewers(value) == 0 {
        issues.push("approved run requires independent reviewer".to_string());
    }
}

fn independent_reviewers(value: &Value) -> usize {
    let producer = text(value, &["producer", "id"]);
    items(value, "reviewers")
        .filter(|item| field_text(item, "id") != producer)
        .count()
}
