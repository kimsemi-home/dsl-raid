use crate::commands::quality::agent_run::fields::{field_is, field_text, items, text};
use serde_json::Value;

const REQUIRED: [&str; 4] = ["reason", "pruned_by", "pruned_at", "policy_hash"];

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    for evidence in items(value, "evidence") {
        if field_is(evidence, "status", "pruned") {
            push_retention_issue(evidence, issues);
            push_tombstone_issues(evidence, issues);
            push_incident_issue(value, evidence, issues);
        }
    }
}

fn push_retention_issue(evidence: &Value, issues: &mut Vec<String>) {
    let Some(retention) = field_text(evidence, "retention") else {
        return;
    };
    if matches!(retention, "protected" | "legal_hold") {
        issues.push(format!(
            "evidence {} retention {retention} blocks pruning",
            id(evidence)
        ));
    }
}

fn push_tombstone_issues(evidence: &Value, issues: &mut Vec<String>) {
    for field in REQUIRED {
        if text(evidence, &["tombstone", field]).is_none() {
            issues.push(format!(
                "pruned evidence {} requires tombstone {field}",
                id(evidence)
            ));
        }
    }
}

fn push_incident_issue(value: &Value, evidence: &Value, issues: &mut Vec<String>) {
    if text(value, &["authority_gate", "scope"]) == Some("incident") {
        issues.push(format!(
            "incident authority cannot prune evidence {}",
            id(evidence)
        ));
    }
}

fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
