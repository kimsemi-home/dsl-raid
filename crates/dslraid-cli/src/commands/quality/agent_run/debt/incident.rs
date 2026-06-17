use crate::commands::quality::agent_run::fields::{items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if text(value, &["authority_gate", "scope"]) == Some("incident")
        && items(value, "debts").next().is_none()
    {
        issues.push("incident authority requires debt record".to_string());
    }
}
