use super::super::id;
use crate::commands::quality::agent_run::fields::field_text;
use serde_json::Value;

pub(super) fn push_issues(claim: &Value, issues: &mut Vec<String>) {
    if field_text(claim, "verification_plan").is_none() {
        issues.push(format!(
            "high confidence claim {} requires verification plan",
            id(claim)
        ));
    }
}
