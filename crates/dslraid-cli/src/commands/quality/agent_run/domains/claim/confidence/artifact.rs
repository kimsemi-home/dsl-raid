use super::super::{evidence, id};
use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    if !mentions_artifact(claim) {
        return;
    }
    let refs = evidence::refs(claim);
    if refs.is_empty() || refs.iter().any(|reference| !has_evidence(value, reference)) {
        return;
    }
    if !refs.iter().any(|reference| has_kind(value, reference)) {
        issues.push(format!(
            "high confidence artifact claim {} requires artifact evidence",
            id(claim)
        ));
    }
}

fn mentions_artifact(value: &Value) -> bool {
    field_text(value, "statement")
        .is_some_and(|text| text.to_ascii_lowercase().contains("artifact"))
}

fn has_evidence(value: &Value, reference: &str) -> bool {
    items(value, "evidence").any(|item| field_text(item, "id") == Some(reference))
}

fn has_kind(value: &Value, reference: &str) -> bool {
    items(value, "evidence")
        .any(|item| field_text(item, "id") == Some(reference) && field_is(item, "kind", "artifact"))
}
