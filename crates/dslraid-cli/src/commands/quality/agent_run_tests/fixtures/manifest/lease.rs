use serde_json::{json, Value};

pub(super) fn record(status: &str) -> Value {
    json!({
        "id": "lease:runscope-quality-001",
        "status": status,
        "ontology_version": "0.1.0"
    })
}
