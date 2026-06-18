use super::super::fields::{field_is, field_text};
use super::evidence;
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn items(value: &Value) -> impl Iterator<Item = &Value> {
    value
        .get("losses")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
}

pub(super) fn push_issues(
    translation_id: &str,
    value: &Value,
    evidence_ids: &BTreeSet<String>,
    issues: &mut Vec<String>,
) {
    push_evidence_issue(value, issues);
    push_forbidden_issue(translation_id, value, issues);
    evidence::push_unknown(
        "loss",
        id(value),
        evidence::refs(value),
        evidence_ids,
        issues,
    );
}

fn push_evidence_issue(value: &Value, issues: &mut Vec<String>) {
    if evidence::refs(value).is_empty() {
        issues.push(format!("loss {} requires evidence", id(value)));
    }
}

fn push_forbidden_issue(translation_id: &str, value: &Value, issues: &mut Vec<String>) {
    if is_forbidden(value) {
        issues.push(format!(
            "translation {translation_id} contains forbidden loss {}",
            id(value)
        ));
    }
}

fn is_forbidden(value: &Value) -> bool {
    field_is(value, "level", "L4") || field_is(value, "status", "forbidden")
}

pub(super) fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
