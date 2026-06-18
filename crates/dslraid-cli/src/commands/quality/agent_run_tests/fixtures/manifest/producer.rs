use serde_json::{json, Value};

pub(super) fn fixture() -> Value {
    json!({
        "id": "agent:codex",
        "role": "implementation",
        "reasoning_level": "R3",
        "trust_tier": "T2"
    })
}
