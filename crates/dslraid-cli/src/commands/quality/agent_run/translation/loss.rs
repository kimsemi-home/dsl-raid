use super::super::fields::{field_is, field_text};
use serde_json::Value;

pub(super) fn items(value: &Value) -> impl Iterator<Item = &Value> {
    value
        .get("losses")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
}

pub(super) fn is_forbidden(value: &Value) -> bool {
    field_is(value, "level", "L4") || field_is(value, "status", "forbidden")
}

pub(super) fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
