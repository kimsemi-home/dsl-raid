mod artifact;
mod links;
mod orchestrator;
mod quality;

use super::{evidence, id};
use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn push_issues(
    value: &Value,
    claim: &Value,
    producer: Option<&str>,
    issues: &mut Vec<String>,
) {
    if !field_is(claim, "confidence", "high") {
        return;
    }
    if evidence::refs(claim).is_empty() {
        issues.push(format!(
            "high confidence claim {} requires evidence",
            id(claim)
        ));
    }
    let Some(assessor) = field_text(claim, "assessor") else {
        issues.push(format!(
            "high confidence claim {} requires assessor",
            id(claim)
        ));
        return;
    };
    if Some(assessor) == producer {
        issues.push(format!(
            "high confidence claim {} cannot be self-assessed",
            id(claim)
        ));
    } else if assessor.starts_with("agent:") {
        issues.push(format!(
            "high confidence claim {} requires external assessor",
            id(claim)
        ));
    }
    push_validation_issue(value, claim, issues);
    artifact::push_issues(value, claim, issues);
    links::push_issues(value, claim, issues);
    orchestrator::push_issues(value, claim, issues);
    quality::push_issues(value, claim, issues);
}

fn push_validation_issue(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    let refs = evidence::refs(claim);
    if refs.is_empty() || refs.iter().any(|reference| !has_evidence(value, reference)) {
        return;
    }
    if !refs
        .iter()
        .any(|reference| has_kind(value, reference, "validation"))
    {
        issues.push(format!(
            "high confidence claim {} requires validation evidence",
            id(claim)
        ));
    }
}

fn has_evidence(value: &Value, reference: &str) -> bool {
    items(value, "evidence").any(|item| field_text(item, "id") == Some(reference))
}

fn has_kind(value: &Value, reference: &str, kind: &str) -> bool {
    items(value, "evidence")
        .any(|item| field_text(item, "id") == Some(reference) && field_is(item, "kind", kind))
}
