mod change;
mod evidence;
mod hash;

use super::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if items(value, "semantic_diffs").next().is_none() {
        issues.push("approved run requires semantic diff receipt".to_string());
    }
    let evidence_ids = evidence::ids(value);
    for item in items(value, "semantic_diffs") {
        push_status_issue(item, issues);
        hash::push_issues(value, item, issues);
        evidence::push_issues(item, &evidence_ids, issues);
        change::push_issues(value, item, issues);
    }
}

fn push_status_issue(item: &Value, issues: &mut Vec<String>) {
    if field_is(item, "status", "blocked") {
        issues.push(format!(
            "approved run cannot use blocked semantic diff {}",
            id(item)
        ));
    }
}

pub(super) fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
