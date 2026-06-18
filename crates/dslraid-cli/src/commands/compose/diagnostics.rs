use serde_json::{json, Value};

pub(super) fn state_space_diagnostics(
    composition_id: &str,
    state_space: usize,
    limit: usize,
) -> Vec<Value> {
    if state_space <= limit {
        return Vec::new();
    }
    vec![json!({
        "code": "CMP026",
        "severity": "warning",
        "message": "Composition state space exceeds materialization limit.",
        "subjects": [composition_id]
    })]
}
