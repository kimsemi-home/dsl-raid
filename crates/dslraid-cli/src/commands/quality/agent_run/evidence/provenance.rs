mod bridge;
mod schema;

use crate::commands::quality::agent_run::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    for evidence in items(value, "evidence") {
        for field in schema::REQUIRED_FIELDS {
            push_required(evidence, field, issues);
        }
        push_kind_issue(evidence, issues);
        push_ontology_issue(value, evidence, issues);
    }
}

fn push_required(evidence: &Value, field: &str, issues: &mut Vec<String>) {
    if text(evidence, &["provenance", field]).is_none() {
        issues.push(format!(
            "evidence {} requires provenance {field}",
            id(evidence)
        ));
    }
}

fn push_kind_issue(evidence: &Value, issues: &mut Vec<String>) {
    let Some(kind) = text(evidence, &["provenance", "kind"]) else {
        return;
    };
    if !schema::supports_kind(kind) {
        issues.push(format!(
            "evidence {} has unsupported provenance kind {kind}",
            id(evidence)
        ));
    }
}

fn push_ontology_issue(value: &Value, evidence: &Value, issues: &mut Vec<String>) {
    let Some(actual) = text(evidence, &["provenance", "ontology_version"]) else {
        return;
    };
    let Some(expected) = text(value, &["ssot", "ontology_version"]) else {
        return;
    };
    if actual != expected && !bridge::covers(value, id(evidence), expected) {
        issues.push(format!(
            "evidence {} provenance ontology {actual} requires translation bridge to ssot {expected}",
            id(evidence)
        ));
    }
}

fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
