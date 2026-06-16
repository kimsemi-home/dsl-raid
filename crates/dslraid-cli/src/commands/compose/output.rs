use anyhow::{anyhow, Result};
use serde_json::Value;

pub(super) fn text(result: &Value, materialize: &str) -> Result<String> {
    let composition = result
        .get("composition")
        .ok_or_else(|| anyhow!("compose result is missing composition"))?;
    let states = result
        .get("states")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let transitions = result
        .get("transitions")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    Ok(format!(
        "composition {} kind={} mode={} state_space={} materialized_states={} materialized_transitions={} truncated={}\n",
        composition
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("<none>"),
        composition
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>"),
        composition
            .get("mode")
            .and_then(Value::as_str)
            .unwrap_or(materialize),
        composition
            .get("state_space")
            .and_then(Value::as_u64)
            .unwrap_or_default(),
        states,
        transitions,
        composition
            .get("truncated")
            .and_then(Value::as_bool)
            .unwrap_or(false)
    ))
}
