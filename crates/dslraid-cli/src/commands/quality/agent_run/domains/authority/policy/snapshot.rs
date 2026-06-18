use crate::commands::quality::agent_run::fields::text;
use serde_json::Value;

const MUTABLE_TOKENS: [&str; 5] = ["latest", "current", "head", "main", "mutable"];

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let Some(policy) = text(value, &["authority_gate", "policy_hash"]) else {
        return;
    };
    if is_mutable(policy) {
        issues.push(format!(
            "approved authority gate cannot use mutable policy pointer {policy}"
        ));
    }
}

fn is_mutable(value: &str) -> bool {
    value.starts_with("policy:") || has_mutable_token(value)
}

fn has_mutable_token(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    MUTABLE_TOKENS.iter().any(|token| value.contains(token))
}
