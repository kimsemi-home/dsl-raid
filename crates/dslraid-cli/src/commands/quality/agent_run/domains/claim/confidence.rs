mod artifact;
mod assessor;
mod links;
mod orchestrator;
mod plan;
mod quality;
mod validation;

use super::{evidence, id};
use crate::commands::quality::agent_run::fields::field_is;
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
    if !assessor::push_issues(claim, producer, issues) {
        return;
    }
    validation::push_issues(value, claim, issues);
    artifact::push_issues(value, claim, issues);
    links::push_issues(value, claim, issues);
    orchestrator::push_issues(value, claim, issues);
    plan::push_issues(claim, issues);
    quality::push_issues(value, claim, issues);
}
