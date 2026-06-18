use super::super::{evidence, id};
use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    let refs = evidence::refs(claim);
    if refs.is_empty() || refs.iter().any(|reference| !has_evidence(value, reference)) {
        return;
    }
    if !refs.iter().any(|reference| has_kind(value, reference)) {
        issues.push(format!(
            "high confidence claim {} requires validation evidence",
            id(claim)
        ));
    }
}

fn has_evidence(value: &Value, reference: &str) -> bool {
    items(value, "evidence").any(|item| field_text(item, "id") == Some(reference))
}

fn has_kind(value: &Value, reference: &str) -> bool {
    items(value, "evidence").any(|item| {
        field_text(item, "id") == Some(reference) && field_is(item, "kind", "validation")
    })
}
