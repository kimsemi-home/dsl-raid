use super::{evidence, id};
use crate::commands::quality::agent_run::fields::{field_is, field_text};
use serde_json::Value;

pub(super) fn push_issues(claim: &Value, producer: Option<&str>, issues: &mut Vec<String>) {
    if !field_is(claim, "confidence", "high") {
        return;
    }
    if evidence::refs(claim).is_empty() {
        issues.push(format!(
            "high confidence claim {} requires evidence",
            id(claim)
        ));
    }
    let Some(assessor) = field_text(claim, "assessor") else {
        issues.push(format!(
            "high confidence claim {} requires assessor",
            id(claim)
        ));
        return;
    };
    if Some(assessor) == producer {
        issues.push(format!(
            "high confidence claim {} cannot be self-assessed",
            id(claim)
        ));
    }
}
