use super::fields::{field_is, field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if has_open_debt(value) {
        issues.push("approved run cannot carry open debt".to_string());
    }
    let evidence = evidence_ids(value);
    for debt in items(value, "debts") {
        push_accountability_issue(debt, issues);
        push_closure_issue(debt, &evidence, issues);
    }
}

fn has_open_debt(value: &Value) -> bool {
    items(value, "debts").any(|item| field_is(item, "status", "open"))
}

fn push_accountability_issue(debt: &Value, issues: &mut Vec<String>) {
    for field in ["owner", "opened_at", "revalidate_at"] {
        if field_text(debt, field).is_none() {
            issues.push(format!("debt {} requires {field}", debt_id(debt)));
        }
    }
}

fn push_closure_issue(debt: &Value, evidence: &BTreeSet<String>, issues: &mut Vec<String>) {
    if !is_closed(debt) {
        return;
    }
    let refs = evidence_refs(debt);
    if refs.is_empty() {
        issues.push(format!("debt {} requires closure evidence", debt_id(debt)));
    }
    for reference in refs {
        if !evidence.contains(reference) {
            issues.push(format!(
                "debt {} references unknown evidence {reference}",
                debt_id(debt)
            ));
        }
    }
}

fn is_closed(debt: &Value) -> bool {
    field_is(debt, "status", "closed") || field_is(debt, "status", "waived")
}

fn evidence_ids(value: &Value) -> BTreeSet<String> {
    items(value, "evidence")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

fn evidence_refs(value: &Value) -> Vec<&str> {
    value
        .get("evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}

fn debt_id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
