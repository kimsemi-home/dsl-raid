mod evidence;
mod participants;

use super::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if !items(value, "agreements").any(is_agree) {
        issues.push("approved run requires cross-agent agreement".to_string());
    }
    let evidence_ids = evidence::ids(value);
    let reviewers = participants::reviewer_ids(value);
    let ontology = text(value, &["ssot", "ontology_version"]);
    for agreement in items(value, "agreements") {
        push_decision_issue(agreement, issues);
        participants::push_issues(agreement, &reviewers, issues);
        push_ontology_issue(agreement, ontology, issues);
        evidence::push_issues(agreement, &evidence_ids, issues);
    }
}

fn push_decision_issue(agreement: &Value, issues: &mut Vec<String>) {
    if !is_agree(agreement) {
        issues.push(format!(
            "approved run cannot carry unresolved agreement {}",
            id(agreement)
        ));
    }
}

fn push_ontology_issue(agreement: &Value, ontology: Option<&str>, issues: &mut Vec<String>) {
    if field_text(agreement, "interpreted_under") != ontology {
        issues.push(format!(
            "agreement {} interpreted_under differs from ssot ontology",
            id(agreement)
        ));
    }
}

fn is_agree(value: &Value) -> bool {
    field_text(value, "decision") == Some("agree")
}

pub(super) fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
