use crate::commands::quality::agent_run::fields::field_text;
use crate::commands::quality::agent_run::reliability::has_authority_subject;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, reviewers: &[&Value], issues: &mut Vec<String>) {
    for reviewer in reviewers.iter().copied().filter(is_trusted) {
        let Some(reviewer_id) = field_text(reviewer, "id") else {
            continue;
        };
        if !has_authority_subject(value, reviewer_id) {
            issues.push(format!(
                "trusted reviewer {reviewer_id} requires reliability evidence"
            ));
        }
    }
}

fn is_trusted(reviewer: &&Value) -> bool {
    matches!(field_text(reviewer, "trust_tier"), Some("T3" | "T4"))
}
