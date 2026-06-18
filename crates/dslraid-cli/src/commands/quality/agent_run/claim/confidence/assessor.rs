use super::super::id;
use crate::commands::quality::agent_run::fields::field_text;
use serde_json::Value;

pub(super) fn push_issues(claim: &Value, producer: Option<&str>, issues: &mut Vec<String>) -> bool {
    let Some(assessor) = field_text(claim, "assessor") else {
        issues.push(format!(
            "high confidence claim {} requires assessor",
            id(claim)
        ));
        return false;
    };
    if Some(assessor) == producer {
        issues.push(format!(
            "high confidence claim {} cannot be self-assessed",
            id(claim)
        ));
    } else if assessor.starts_with("agent:") {
        issues.push(format!(
            "high confidence claim {} requires external assessor",
            id(claim)
        ));
    }
    true
}
