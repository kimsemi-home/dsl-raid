use crate::commands::quality::agent_run::fields::text;
use crate::commands::quality::agent_run::reliability::has_authority_subject;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if !matches!(text(value, &["producer", "trust_tier"]), Some("T3" | "T4")) {
        return;
    }
    let Some(producer) = text(value, &["producer", "id"]) else {
        return;
    };
    if !has_authority_subject(value, producer) {
        issues.push(format!(
            "trusted producer {producer} requires reliability evidence"
        ));
    }
}
