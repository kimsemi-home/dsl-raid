use super::super::fields::{field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn ids(value: &Value) -> BTreeSet<String> {
    items(value, "evidence")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

pub(super) fn refs(value: &Value) -> Vec<&str> {
    value
        .get("evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}

pub(super) fn push_unknown(
    kind: &str,
    id: &str,
    refs: Vec<&str>,
    evidence: &BTreeSet<String>,
    issues: &mut Vec<String>,
) {
    for reference in refs {
        if !evidence.contains(reference) {
            issues.push(format!(
                "{kind} {id} references unknown evidence {reference}"
            ));
        }
    }
}
