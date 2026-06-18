use serde_json::Value;

pub(super) fn has(value: &Value, reference: &str) -> bool {
    value
        .get("evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|item| item.as_str() == Some(reference))
}

pub(super) fn ids(value: &Value) -> Vec<&str> {
    value
        .get("evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}
