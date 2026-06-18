use serde_json::Value;

pub(super) fn has_prior(value: &Value) -> bool {
    value
        .get("supersedes")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|item| item.as_str().is_some())
}
