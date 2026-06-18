use crate::commands::quality::agent_run::fields::{field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, reviewers: &[&Value], issues: &mut Vec<String>) {
    if !is_high_risk(value) {
        return;
    }
    if !reviewers
        .iter()
        .any(|item| field_text(item, "role") == Some("adversarial-review"))
    {
        issues.push("high-risk authority requires adversarial reviewer".to_string());
    }
}

fn is_high_risk(value: &Value) -> bool {
    matches!(
        text(value, &["authority_gate", "scope"]),
        Some("security" | "audit" | "ontology" | "incident" | "authority")
    )
}
