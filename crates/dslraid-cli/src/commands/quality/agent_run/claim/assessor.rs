use super::id;
use crate::commands::quality::agent_run::fields::{field_is, field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    if !field_is(claim, "status", "supported") || field_is(claim, "confidence", "high") {
        return;
    }
    let Some(assessor) = field_text(claim, "assessor") else {
        return;
    };
    if text(value, &["producer", "id"]) == Some(assessor) {
        issues.push(format!(
            "supported claim {} cannot be self-assessed by producer {assessor}",
            id(claim)
        ));
    }
    if text(value, &["orchestration", "routed_by"]) == Some(assessor) {
        issues.push(format!(
            "supported claim {} cannot be assessed by control plane {assessor}",
            id(claim)
        ));
    }
}
