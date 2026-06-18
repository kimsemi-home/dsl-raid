mod actors;
mod authority;

use crate::commands::quality::agent_run::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let Some(run_id) = text(value, &["run", "id"]) else {
        return;
    };
    let actors = actors::collect(value);
    let refs = authority::refs(value);
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
    actors: &authority::Actors,
    refs: &authority::Refs,
) -> bool {
    field_text(evidence, "subject") == Some(run_id)
        || authority::actor_subject(evidence, actors, refs)
}

fn evidence_id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
