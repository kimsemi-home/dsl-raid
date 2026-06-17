use crate::commands::quality::agent_run::fields::{field_text, items, text};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    let routed_by = field_text(item, "routed_by");
    let verified_by = field_text(item, "verified_by");
    push_required(routed_by, "routed_by", issues);
    push_required(verified_by, "verified_by", issues);
    let Some(verifier) = verified_by else {
        return;
    };
    push_separation(verifier, routed_by, "orchestrator", issues);
    push_separation(
        verifier,
        selected_producer(item),
        "selected producer",
        issues,
    );
    push_separation(verifier, producer(value), "manifest producer", issues);
    push_known_reviewer(value, verifier, issues);
}

fn push_required(value: Option<&str>, key: &str, issues: &mut Vec<String>) {
    if value.is_none() {
        issues.push(format!("orchestration receipt requires {key}"));
    }
}

fn push_separation(verifier: &str, other: Option<&str>, label: &str, issues: &mut Vec<String>) {
    if other == Some(verifier) {
        issues.push(format!(
            "control plane verifier cannot be {label} {verifier}"
        ));
    }
}

fn push_known_reviewer(value: &Value, verifier: &str, issues: &mut Vec<String>) {
    let reviewers = reviewer_ids(value);
    if !reviewers.is_empty() && !reviewers.contains(verifier) {
        issues.push(format!(
            "control plane verifier {verifier} must be a reviewer"
        ));
    }
}

fn reviewer_ids(value: &Value) -> BTreeSet<String> {
    items(value, "reviewers")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

fn selected_producer(value: &Value) -> Option<&str> {
    field_text(value, "selected_producer")
}

fn producer(value: &Value) -> Option<&str> {
    text(value, &["producer", "id"])
}
