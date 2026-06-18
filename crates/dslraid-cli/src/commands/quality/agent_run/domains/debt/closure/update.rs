use super::{evidence, id};
use crate::commands::quality::agent_run::fields::{field_is, field_text};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(debt: &Value, evidence_ids: &BTreeSet<String>, issues: &mut Vec<String>) {
    let updates = updates(debt);
    if updates.is_empty() {
        issues.push(format!(
            "debt {} requires feedback closure update",
            id(debt)
        ));
    }
    for update in updates {
        push_status_issue(debt, update, issues);
        push_evidence_issues(debt, update, evidence_ids, issues);
    }
}

fn push_status_issue(debt: &Value, update: &Value, issues: &mut Vec<String>) {
    if field_is(update, "status", "proposed") {
        issues.push(format!(
            "debt {} has unapplied feedback update {}",
            id(debt),
            field_text(update, "id").unwrap_or("<unknown>")
        ));
    }
}

fn push_evidence_issues(
    debt: &Value,
    update: &Value,
    evidence_ids: &BTreeSet<String>,
    issues: &mut Vec<String>,
) {
    let refs = evidence::refs(update);
    let update_id = field_text(update, "id").unwrap_or(id(debt));
    if refs.is_empty() {
        issues.push(format!("feedback update {update_id} requires evidence"));
    }
    for reference in refs {
        if !evidence_ids.contains(reference) {
            issues.push(format!(
                "feedback update {} references unknown evidence {reference}",
                update_id
            ));
        }
    }
}

fn updates(value: &Value) -> Vec<&Value> {
    value
        .get("updates")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .collect()
}
