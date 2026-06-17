mod abort;
mod evidence;
mod quarantine;
mod release;
mod required;
mod subject;

use super::fields::{field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    required::push_issues(value, issues);
    abort::push_issues(value, issues);
    quarantine::push_issues(value, issues);
    let evidence_ids = evidence::ids(value);
    for item in items(value, "containments") {
        subject::push_issues(value, item, issues);
        push_accountability_issue(item, issues);
        let refs = evidence::refs(item);
        evidence::push_unknown("containment", id(item), refs, &evidence_ids, issues);
        release::push_issues(item, &evidence_ids, issues);
    }
}

fn push_accountability_issue(item: &Value, issues: &mut Vec<String>) {
    for field in ["owner", "opened_at"] {
        if field_text(item, field).is_none() {
            issues.push(format!("containment {} requires {field}", id(item)));
        }
    }
    if evidence::refs(item).is_empty() {
        issues.push(format!("containment {} requires evidence", id(item)));
    }
}

pub(super) fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
