mod adversarial;
mod isolation;

use super::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let reviewers = independent_reviewers(value);
    if reviewers.is_empty() {
        issues.push("approved run requires independent reviewer".to_string());
    }
    isolation::push_issues(value, issues);
    adversarial::push_issues(value, &reviewers, issues);
    for reviewer in reviewers {
        push_required_issue(reviewer, "role", "role", issues);
        push_required_issue(reviewer, "reasoning_level", "reasoning level", issues);
        push_required_issue(reviewer, "trust_tier", "trust tier", issues);
        push_cold_start_issue(reviewer, issues);
    }
}

fn independent_reviewers(value: &Value) -> Vec<&Value> {
    let producer = text(value, &["producer", "id"]);
    items(value, "reviewers")
        .filter(|item| field_text(item, "id") != producer)
        .collect()
}

fn push_required_issue(reviewer: &Value, key: &str, label: &str, issues: &mut Vec<String>) {
    if field_text(reviewer, key).is_none() {
        issues.push(format!(
            "independent reviewer {} requires {label}",
            id(reviewer)
        ));
    }
}

fn push_cold_start_issue(reviewer: &Value, issues: &mut Vec<String>) {
    if matches!(field_text(reviewer, "trust_tier"), Some("T0" | "T1")) {
        issues.push(format!(
            "approved run cannot use cold-start reviewer {}",
            id(reviewer)
        ));
    }
}

fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
