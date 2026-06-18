use serde_json::Value;
use std::collections::BTreeMap;

pub(crate) fn artifact_map(lock_value: &Value) -> BTreeMap<String, Value> {
    lock_value
        .get("artifacts")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|artifact| {
            artifact
                .get("artifact")
                .and_then(Value::as_str)
                .map(|id| (id.to_string(), artifact.clone()))
        })
        .collect()
}

pub(crate) fn derivation_input_map(lock_value: &Value) -> BTreeMap<String, String> {
    lock_value
        .get("derivations")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(derivation_input)
        .collect()
}

fn derivation_input(derivation: &Value) -> Option<(String, String)> {
    let id = derivation.get("derivation").and_then(Value::as_str)?;
    let input_hash = derivation.get("input_hash").and_then(Value::as_str)?;
    Some((id.to_string(), input_hash.to_string()))
}
