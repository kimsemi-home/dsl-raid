use super::id;
use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn ids(value: &Value) -> BTreeSet<String> {
    items(value, "evidence")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

pub(super) fn push_issues(debt: &Value, evidence: &BTreeSet<String>, issues: &mut Vec<String>) {
    let refs = refs(debt);
    if refs.is_empty() {
        issues.push(format!("debt {} requires closure evidence", id(debt)));
    }
    for reference in refs {
        if !evidence.contains(reference) {
            issues.push(format!(
                "debt {} references unknown evidence {reference}",
                id(debt)
            ));
        }
    }
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
