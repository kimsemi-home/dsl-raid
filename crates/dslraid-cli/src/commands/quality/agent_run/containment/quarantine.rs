use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if !has_open_quarantine(value) {
        return;
    }
    for artifact in items(value, "artifacts").filter(|item| field_is(item, "status", "verified")) {
        issues.push(format!(
            "open quarantine blocks verified artifact {}",
            id(artifact)
        ));
    }
    for claim in items(value, "claims").filter(|item| field_is(item, "confidence", "high")) {
        issues.push(format!(
            "open quarantine blocks high confidence claim {}",
            id(claim)
        ));
    }
}

fn has_open_quarantine(value: &Value) -> bool {
    items(value, "containments")
        .any(|item| field_is(item, "kind", "quarantine") && field_is(item, "status", "open"))
}

fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
