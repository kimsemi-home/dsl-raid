use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn item_string(item: &Value, key: &str) -> String {
    item.get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

pub(super) fn changed_fields(before: &Value, after: &Value) -> Vec<String> {
    let mut keys = BTreeSet::new();
    if let Some(object) = before.as_object() {
        keys.extend(object.keys().cloned());
    }
    if let Some(object) = after.as_object() {
        keys.extend(object.keys().cloned());
    }
    keys.into_iter()
        .filter(|key| before.get(key) != after.get(key))
        .collect()
}

pub(super) fn is_terminal_state_item(item: &Value) -> bool {
    item.get("kind").and_then(Value::as_str) == Some("state")
        && item
            .get("terminal")
            .and_then(Value::as_bool)
            .unwrap_or(false)
}

pub(super) fn has_policy_trace(item: &Value) -> bool {
    item.get("requires")
        .and_then(Value::as_array)
        .is_some_and(|requires| !requires.is_empty())
}
