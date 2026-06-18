use crate::commands::quality::agent_run::fields::{field_is, field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let Some(status) = text(value, &["lease", "status"]) else {
        return;
    };
    if status == "finished" {
        return;
    }
    for artifact in items(value, "artifacts").filter(|item| field_is(item, "status", "verified")) {
        issues.push(format!(
            "{status} lease blocks verified artifact {}",
            id(artifact)
        ));
    }
}

fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
