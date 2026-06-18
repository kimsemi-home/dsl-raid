use super::super::super::fields::{field_is, field_text};
use serde_json::Value;

pub(super) fn collect(value: &Value) -> Vec<&Value> {
    value
        .get("quality_snapshots")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .collect()
}

pub(super) fn has_high(values: &[&Value]) -> bool {
    values.iter().any(is_high)
}

pub(super) fn has_assessor(values: &[&Value], assessor: &str) -> bool {
    values
        .iter()
        .any(|value| field_text(value, "assessor") == Some(assessor))
}

fn is_high(value: &&Value) -> bool {
    field_is(value, "quality", "high")
}
