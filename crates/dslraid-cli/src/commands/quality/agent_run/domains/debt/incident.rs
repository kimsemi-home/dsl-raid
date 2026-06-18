use crate::commands::quality::agent_run::fields::{field_is, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if text(value, &["authority_gate", "scope"]) != Some("incident") {
        return;
    }
    if items(value, "debts").next().is_none() {
        issues.push("incident authority requires debt record".to_string());
        return;
    }
    if !items(value, "debts").any(|item| field_is(item, "kind", "evidence")) {
        issues.push("incident authority requires evidence debt record".to_string());
    }
}
