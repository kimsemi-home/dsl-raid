use super::super::fields::{field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    let id = field_text(item, "id").unwrap_or("<unknown>");
    let Some(subject) = field_text(item, "subject") else {
        issues.push(format!("containment {id} requires subject"));
        return;
    };
    let Some(run_id) = text(value, &["run", "id"]) else {
        return;
    };
    if subject != run_id {
        issues.push(format!(
            "containment {id} subject {subject} must match run {run_id}"
        ));
    }
}
