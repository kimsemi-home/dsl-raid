use super::super::{evidence, id};
use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    for reference in evidence::refs(claim) {
        let Some(item) = evidence_item(value, reference) else {
            continue;
        };
        if items(item, "links").next().is_none() {
            issues.push(format!(
                "high confidence claim {} requires linked evidence {reference}",
                id(claim)
            ));
        }
    }
}

fn evidence_item<'a>(value: &'a Value, reference: &str) -> Option<&'a Value> {
    items(value, "evidence").find(|item| field_text(item, "id") == Some(reference))
}
