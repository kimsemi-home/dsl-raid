use super::fields::{field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let mut has_artifact = false;
    for artifact in items(value, "artifacts") {
        has_artifact = true;
        push_status_issue(artifact, issues);
    }
    if !has_artifact {
        issues.push("approved run requires output artifact record".to_string());
    }
}

fn push_status_issue(artifact: &Value, issues: &mut Vec<String>) {
    match field_text(artifact, "status") {
        Some("verified") | None => {}
        Some(status) => issues.push(format!(
            "approved run cannot carry {status} artifact {}",
            id(artifact)
        )),
    }
}

fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
