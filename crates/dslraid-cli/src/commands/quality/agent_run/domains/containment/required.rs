use super::super::fields::{field_is, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if aborted_signal(value) && !has_kind(value, "abort") {
        issues.push("aborted run requires abort evidence bundle".to_string());
    }
    if quarantined_signal(value) && !has_kind(value, "quarantine") {
        issues.push("quarantined output requires quarantine evidence bundle".to_string());
    }
}

pub(super) fn aborted_signal(value: &Value) -> bool {
    text(value, &["run", "status"]) == Some("aborted")
        || text(value, &["lease", "status"]) == Some("aborted")
        || items(value, "artifacts").any(|item| field_is(item, "status", "aborted"))
}

fn quarantined_signal(value: &Value) -> bool {
    text(value, &["run", "status"]) == Some("quarantined")
        || text(value, &["lease", "status"]) == Some("quarantined")
        || items(value, "artifacts").any(|item| field_is(item, "status", "quarantined"))
}

fn has_kind(value: &Value, kind: &str) -> bool {
    items(value, "containments").any(|item| field_is(item, "kind", kind))
}
