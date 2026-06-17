use serde_json::{json, Value};

const RUN_ID: &str = "agent-run:runscope-quality-001";

pub(super) fn with_subject(value: &mut Value) {
    let Some(items) = value.as_array_mut() else {
        return;
    };
    for item in items {
        if item.get("subject").is_none() {
            item["subject"] = json!(RUN_ID);
        }
    }
}

pub(super) fn high() -> Value {
    let snapshot = high_snapshot();
    json!([
        { "id": "evidence:quality", "quality": "high", "kind": "validation", "quality_snapshots": snapshot.clone() },
        { "id": "evidence:trace", "quality": "high", "kind": "trace", "quality_snapshots": snapshot.clone() },
        { "id": "evidence:coverage", "quality": "high", "kind": "coverage", "quality_snapshots": snapshot }
    ])
}

pub(super) fn high_snapshot() -> Value {
    json!([{
        "assessed_at": "2026-06-17T00:00:00Z",
        "assessor": "sidecar:dslraid-quality",
        "purpose": "authority",
        "quality": "high",
        "ontology_version": "0.1.0"
    }])
}

pub(super) fn fresh_lock() -> Value {
    json!({
        "core": { "ir_hash": "sha256:core" },
        "artifacts": [
            { "path": "generated/runtime_fsm.rs", "status": "fresh" }
        ]
    })
}
