use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) type Ids = BTreeSet<String>;

pub(super) fn ids(value: &Value) -> Ids {
    items(value, "evidence")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

pub(super) fn gap_ids(value: &Value) -> Ids {
    items(value, "evidence")
        .filter(|item| field_is(item, "kind", "debt") && !field_is(item, "status", "pruned"))
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

pub(super) fn refs(value: &Value) -> Vec<&str> {
    value
        .get("evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}
