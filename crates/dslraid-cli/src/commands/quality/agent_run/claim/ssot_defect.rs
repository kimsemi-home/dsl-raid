use super::id;
use crate::commands::quality::agent_run::fields::{field_is, field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    if !is_supported_ssot_defect(claim) {
        return;
    }
    if text(value, &["authority_gate", "scope"]) != Some("authority") {
        issues.push(format!(
            "ssot defect claim {} requires authority governance scope",
            id(claim)
        ));
    }
}

fn is_supported_ssot_defect(value: &Value) -> bool {
    field_is(value, "status", "supported")
        && field_text(value, "statement")
            .is_some_and(|text| text.to_ascii_lowercase().contains("ssot defect"))
}
