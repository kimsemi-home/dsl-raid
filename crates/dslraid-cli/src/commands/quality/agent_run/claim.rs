mod confidence;
mod evidence;

use super::fields::{field_is, field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let evidence_ids = evidence::ids(value);
    let producer = text(value, &["producer", "id"]);
    for claim in items(value, "claims") {
        push_interpreter_issue(claim, issues);
        push_supported_issue(claim, issues);
        confidence::push_issues(claim, producer, issues);
        evidence::push_unknown_refs(claim, &evidence_ids, issues);
    }
}

fn push_interpreter_issue(claim: &Value, issues: &mut Vec<String>) {
    if field_text(claim, "interpreted_under").is_none() {
        issues.push(format!("claim {} requires interpreted_under", id(claim)));
    }
}

fn push_supported_issue(claim: &Value, issues: &mut Vec<String>) {
    if field_is(claim, "status", "supported") && evidence::refs(claim).is_empty() {
        issues.push(format!("supported claim {} requires evidence", id(claim)));
    }
}

pub(super) fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
