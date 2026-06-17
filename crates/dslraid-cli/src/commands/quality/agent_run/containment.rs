mod evidence;
mod required;

use super::fields::{field_is, field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    required::push_issues(value, issues);
    let evidence_ids = evidence::ids(value);
    for item in items(value, "containments") {
        push_accountability_issue(item, issues);
        let refs = evidence::refs(item);
        evidence::push_unknown("containment", id(item), refs, &evidence_ids, issues);
        push_release_issues(item, &evidence_ids, issues);
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

fn push_release_issues(item: &Value, evidence: &BTreeSet<String>, issues: &mut Vec<String>) {
    if !field_is(item, "status", "released") {
        return;
    }
    if release_conditions(item).next().is_none() {
        issues.push(format!(
            "released containment {} requires release conditions",
            id(item)
        ));
    }
    for condition in release_conditions(item) {
        if !field_is(condition, "status", "met") {
            issues.push(format!(
                "released containment {} has unmet release condition",
                id(item)
            ));
        }
        if evidence::refs(condition).is_empty() {
            issues.push(format!(
                "release condition {} requires evidence",
                id(condition)
            ));
        }
        let refs = evidence::refs(condition);
        evidence::push_unknown("release condition", id(condition), refs, evidence, issues);
    }
}

fn release_conditions(value: &Value) -> impl Iterator<Item = &Value> {
    value
        .get("release_conditions")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
}

pub(super) fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
