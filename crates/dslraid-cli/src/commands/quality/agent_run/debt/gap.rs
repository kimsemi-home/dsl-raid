use super::super::fields::{field_is, field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let evidence_ids = evidence_ids(value);
    let gap_ids = gap_evidence_ids(value);
    for debt in items(value, "debts").filter(|item| field_is(item, "status", "open")) {
        push_evidence_issues(debt, &evidence_ids, &gap_ids, issues);
    }
}

fn push_evidence_issues(
    debt: &Value,
    evidence_ids: &BTreeSet<String>,
    gap_ids: &BTreeSet<String>,
    issues: &mut Vec<String>,
) {
    let refs = refs(debt);
    if refs.is_empty() || refs.iter().all(|reference| !gap_ids.contains(*reference)) {
        issues.push(format!("open debt {} requires loop gap evidence", id(debt)));
    }
    for reference in refs {
        if !evidence_ids.contains(reference) {
            issues.push(format!(
                "open debt {} references unknown evidence {reference}",
                id(debt)
            ));
        }
    }
}

fn evidence_ids(value: &Value) -> BTreeSet<String> {
    items(value, "evidence")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

fn gap_evidence_ids(value: &Value) -> BTreeSet<String> {
    items(value, "evidence")
        .filter(|item| field_is(item, "kind", "debt") && !field_is(item, "status", "pruned"))
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

fn refs(value: &Value) -> Vec<&str> {
    value
        .get("evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}

fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
