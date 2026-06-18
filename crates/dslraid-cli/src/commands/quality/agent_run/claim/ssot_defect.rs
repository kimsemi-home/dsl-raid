use super::id;
use crate::commands::quality::agent_run::fields::{field_is, field_text, items, text};
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
    if field_text(claim, "verification_plan").is_none() {
        issues.push(format!(
            "ssot defect claim {} requires verification plan",
            id(claim)
        ));
    }
    if !has_open_quarantine(value) {
        issues.push(format!(
            "ssot defect claim {} requires open quarantine containment",
            id(claim)
        ));
    }
}

fn is_supported_ssot_defect(value: &Value) -> bool {
    field_is(value, "status", "supported")
        && field_text(value, "statement")
            .is_some_and(|text| text.to_ascii_lowercase().contains("ssot defect"))
}

fn has_open_quarantine(value: &Value) -> bool {
    items(value, "containments")
        .any(|item| field_is(item, "kind", "quarantine") && field_is(item, "status", "open"))
}
