use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, reviewers: &[&Value], issues: &mut Vec<String>) {
    for reviewer in reviewers.iter().copied().filter(is_trusted) {
        let Some(reviewer_id) = field_text(reviewer, "id") else {
            continue;
        };
        if !has_authority_evidence(value, reviewer_id) {
            issues.push(format!(
                "trusted reviewer {reviewer_id} requires reliability evidence"
            ));
        }
    }
}

fn is_trusted(reviewer: &&Value) -> bool {
    matches!(field_text(reviewer, "trust_tier"), Some("T3" | "T4"))
}

fn has_authority_evidence(value: &Value, reviewer_id: &str) -> bool {
    authority_refs(value).iter().any(|reference| {
        items(value, "evidence").any(|evidence| {
            field_text(evidence, "id") == Some(reference.as_str())
                && field_text(evidence, "subject") == Some(reviewer_id)
        })
    })
}

fn authority_refs(value: &Value) -> Vec<String> {
    value
        .pointer("/authority_gate/evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}
