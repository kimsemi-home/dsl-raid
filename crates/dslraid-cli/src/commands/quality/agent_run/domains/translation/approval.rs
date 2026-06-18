use crate::commands::quality::agent_run::fields::{field_is, field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    if !requires_approval(item) {
        return;
    }
    let id = field_text(item, "id").unwrap_or("<unknown>");
    let Some(approver) = field_text(item, "approved_by") else {
        issues.push(format!("translation {id} requires approver"));
        return;
    };
    if Some(approver) == text(value, &["producer", "id"]) {
        issues.push(format!("translation {id} cannot be self-approved"));
    }
}

fn requires_approval(value: &Value) -> bool {
    field_is(value, "status", "lossy") || field_is(value, "status", "verified")
}
