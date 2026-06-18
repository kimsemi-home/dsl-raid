use crate::commands::quality::agent_run::fields::{field_text, items, text};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn collect(value: &Value) -> BTreeSet<String> {
    let mut actors = BTreeSet::new();
    push_path(value, &["authority_gate", "approved_by"], &mut actors);
    push_path(value, &["producer", "id"], &mut actors);
    for reviewer in items(value, "reviewers") {
        if let Some(id) = field_text(reviewer, "id") {
            actors.insert(id.to_string());
        }
    }
    actors
}

fn push_path(value: &Value, path: &[&str], actors: &mut BTreeSet<String>) {
    if let Some(id) = text(value, path) {
        actors.insert(id.to_string());
    }
}
