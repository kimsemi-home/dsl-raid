use serde_json::{json, Value};

pub(super) fn fixture() -> Value {
    json!([{
        "id": "artifact:runtime-rust",
        "path": "generated/runtime_fsm.rs",
        "status": "verified"
    }])
}
