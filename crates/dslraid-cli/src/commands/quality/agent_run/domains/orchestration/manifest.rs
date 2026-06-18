use crate::commands::quality::agent_run::fields::{field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    push_match(
        item,
        "selected_producer",
        producer(value),
        "producer",
        issues,
    );
    push_match(
        item,
        "authority_profile",
        profile(value),
        "authority profile",
        issues,
    );
    push_match(item, "policy_hash", policy(value), "policy hash", issues);
    push_match(item, "lease", lease(value), "lease", issues);
}

fn push_match(
    item: &Value,
    key: &str,
    expected: Option<&str>,
    label: &str,
    issues: &mut Vec<String>,
) {
    let Some(actual) = field_text(item, key) else {
        issues.push(format!("orchestration receipt requires {key}"));
        return;
    };
    if let Some(expected) = expected {
        if expected != actual {
            issues.push(format!(
                "orchestration {label} {actual} differs from manifest {expected}"
            ));
        }
    }
}

fn producer(value: &Value) -> Option<&str> {
    text(value, &["producer", "id"])
}

fn profile(value: &Value) -> Option<&str> {
    text(value, &["authority_gate", "profile"])
}

fn policy(value: &Value) -> Option<&str> {
    text(value, &["authority_gate", "policy_hash"])
}

fn lease(value: &Value) -> Option<&str> {
    text(value, &["lease", "id"])
}
