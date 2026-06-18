use crate::commands::quality::agent_run::fields::field_text;
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) type Actors = BTreeSet<String>;
pub(super) type Refs = BTreeSet<String>;

pub(super) fn refs(value: &Value) -> Refs {
    value
        .pointer("/authority_gate/evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

pub(super) fn actor_subject(evidence: &Value, actors: &Actors, refs: &Refs) -> bool {
    let Some(subject) = field_text(evidence, "subject") else {
        return false;
    };
    actors.contains(subject) && refs.contains(evidence_id(evidence))
}

fn evidence_id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
