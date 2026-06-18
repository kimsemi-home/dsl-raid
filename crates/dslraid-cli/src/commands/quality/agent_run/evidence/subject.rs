use crate::commands::quality::agent_run::fields::{field_text, items, text};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let Some(run_id) = text(value, &["run", "id"]) else {
        return;
    };
    let actors = authority_actors(value);
    let refs = authority_refs(value);
    for evidence in items(value, "evidence") {
        if subject_matches(evidence, run_id, &actors, &refs) {
            continue;
        }
        if let Some(subject) = field_text(evidence, "subject") {
            issues.push(format!(
                "evidence {} subject {subject} is not authorized for run {run_id}",
                evidence_id(evidence)
            ));
        }
    }
}

fn subject_matches(
    evidence: &Value,
    run_id: &str,
    actors: &BTreeSet<String>,
    refs: &BTreeSet<String>,
) -> bool {
    field_text(evidence, "subject") == Some(run_id)
        || authority_actor_subject(evidence, actors, refs)
}

fn authority_actor_subject(
    evidence: &Value,
    actors: &BTreeSet<String>,
    refs: &BTreeSet<String>,
) -> bool {
    let Some(subject) = field_text(evidence, "subject") else {
        return false;
    };
    actors.contains(subject) && refs.contains(evidence_id(evidence))
}

fn authority_actors(value: &Value) -> BTreeSet<String> {
    [
        text(value, &["authority_gate", "approved_by"]),
        text(value, &["producer", "id"]),
    ]
    .into_iter()
    .flatten()
    .map(str::to_string)
    .collect()
}

fn authority_refs(value: &Value) -> BTreeSet<String> {
    value
        .pointer("/authority_gate/evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn evidence_id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
