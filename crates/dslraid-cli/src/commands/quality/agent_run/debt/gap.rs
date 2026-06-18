mod evidence;

use super::super::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let evidence_ids = evidence::ids(value);
    let gap_ids = evidence::gap_ids(value);
    for debt in items(value, "debts").filter(|item| field_is(item, "status", "open")) {
        push_evidence_issues(debt, &evidence_ids, &gap_ids, issues);
    }
}

fn push_evidence_issues(
    debt: &Value,
    evidence_ids: &evidence::Ids,
    gap_ids: &evidence::Ids,
    issues: &mut Vec<String>,
) {
    let refs = evidence::refs(debt);
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

fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
