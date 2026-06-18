use crate::commands::quality::agent_run::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn selected_producer(value: &Value) -> Option<&str> {
    field_text(value, "selected_producer")
}

pub(super) fn manifest_producer(value: &Value) -> Option<&str> {
    text(value, &["producer", "id"])
}

pub(super) fn has_reviewer_ids(value: &Value) -> bool {
    items(value, "reviewers").any(|item| field_text(item, "id").is_some())
}

pub(super) fn contains_reviewer(value: &Value, verifier: &str) -> bool {
    items(value, "reviewers").any(|item| field_text(item, "id") == Some(verifier))
}
