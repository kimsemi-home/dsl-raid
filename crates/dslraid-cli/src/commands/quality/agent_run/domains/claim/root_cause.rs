use super::{evidence, id};
use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    if !is_supported_root_cause(claim) {
        return;
    }
    let refs = evidence::refs(claim);
    if refs.is_empty() {
        return;
    }
    if !refs
        .iter()
        .any(|reference| has_validation(value, reference))
    {
        issues.push(format!(
            "root cause claim {} requires validation evidence",
            id(claim)
        ));
    }
    if !refs
        .iter()
        .any(|reference| has_root_snapshot(value, reference))
    {
        issues.push(format!(
            "root cause claim {} requires root_cause quality snapshot",
            id(claim)
        ));
    }
}

fn is_supported_root_cause(value: &Value) -> bool {
    field_is(value, "status", "supported")
        && field_text(value, "statement")
            .is_some_and(|text| text.to_ascii_lowercase().contains("root cause"))
}

fn has_validation(value: &Value, reference: &str) -> bool {
    evidence_item(value, reference).is_some_and(|item| field_is(item, "kind", "validation"))
}

fn has_root_snapshot(value: &Value, reference: &str) -> bool {
    evidence_item(value, reference).is_some_and(|item| {
        items(item, "quality_snapshots").any(|snapshot| field_is(snapshot, "purpose", "root_cause"))
    })
}

fn evidence_item<'a>(value: &'a Value, reference: &str) -> Option<&'a Value> {
    items(value, "evidence").find(|item| field_text(item, "id") == Some(reference))
}
