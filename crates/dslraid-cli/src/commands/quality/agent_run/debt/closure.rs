mod evidence;
mod learning;
mod update;

use crate::commands::quality::agent_run::fields::{field_is, field_text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, debt: &Value, issues: &mut Vec<String>) {
    push_accountability_issue(debt, issues);
    if !is_closed(debt) {
        return;
    }
    let evidence_ids = evidence::ids(value);
    evidence::push_issues(debt, &evidence_ids, issues);
    learning::push_issues(debt, issues);
    update::push_issues(debt, &evidence_ids, issues);
}

fn push_accountability_issue(debt: &Value, issues: &mut Vec<String>) {
    for field in ["owner", "opened_at", "revalidate_at"] {
        if field_text(debt, field).is_none() {
            issues.push(format!("debt {} requires {field}", id(debt)));
        }
    }
}

fn is_closed(debt: &Value) -> bool {
    field_is(debt, "status", "closed") || field_is(debt, "status", "waived")
}

pub(super) fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
