use super::super::id;
use crate::commands::quality::agent_run::fields::{field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    let Some(assessor) = field_text(claim, "assessor") else {
        return;
    };
    let Some(orchestrator) = text(value, &["orchestration", "routed_by"]) else {
        return;
    };
    if assessor == orchestrator {
        issues.push(format!(
            "high confidence claim {} cannot be assessed by control plane {orchestrator}",
            id(claim)
        ));
    }
}
