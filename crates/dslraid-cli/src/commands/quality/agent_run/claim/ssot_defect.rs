mod retrospective;

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
    if !has_quarantine_containment(value) {
        issues.push(format!(
            "ssot defect claim {} requires quarantine containment record",
            id(claim)
        ));
    }
    if !has_changed_semantic_diff(value) {
        issues.push(format!(
            "ssot defect claim {} requires changed semantic diff",
            id(claim)
        ));
    }
    if !has_described_changed_semantic_diff(value) {
        issues.push(format!(
            "ssot defect claim {} requires changed semantic diff summary",
            id(claim)
        ));
    }
    if !retrospective::has_linked_review_debt(value, claim) {
        issues.push(format!(
            "ssot defect claim {} requires linked closed review debt",
            id(claim)
        ));
    }
}

fn is_supported_ssot_defect(value: &Value) -> bool {
    field_is(value, "status", "supported")
        && field_text(value, "statement")
            .is_some_and(|text| text.to_ascii_lowercase().contains("ssot defect"))
}

fn has_quarantine_containment(value: &Value) -> bool {
    items(value, "containments").any(|item| field_is(item, "kind", "quarantine"))
}

fn has_changed_semantic_diff(value: &Value) -> bool {
    items(value, "semantic_diffs").any(|item| field_is(item, "status", "changed"))
}

fn has_described_changed_semantic_diff(value: &Value) -> bool {
    items(value, "semantic_diffs")
        .any(|item| field_is(item, "status", "changed") && field_text(item, "summary").is_some())
}
