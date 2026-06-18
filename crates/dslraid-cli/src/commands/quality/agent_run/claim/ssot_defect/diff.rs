use super::super::id;
use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    if !has_changed(value) {
        issues.push(format!(
            "ssot defect claim {} requires changed semantic diff",
            id(claim)
        ));
    }
    if !has_summary(value) {
        issues.push(format!(
            "ssot defect claim {} requires changed semantic diff summary",
            id(claim)
        ));
    }
}

fn has_changed(value: &Value) -> bool {
    items(value, "semantic_diffs").any(|item| field_is(item, "status", "changed"))
}

fn has_summary(value: &Value) -> bool {
    items(value, "semantic_diffs")
        .any(|item| field_is(item, "status", "changed") && field_text(item, "summary").is_some())
}
