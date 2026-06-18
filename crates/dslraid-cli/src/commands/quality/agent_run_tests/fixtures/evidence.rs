use serde_json::{json, Value};

const RUN_ID: &str = "agent-run:runscope-quality-001";

pub(super) fn with_subject(value: &mut Value) {
    let Some(items) = value.as_array_mut() else {
        return;
    };
    for (index, item) in items.iter_mut().enumerate() {
        if item.get("id").is_none() {
            item["id"] = json!(format!("evidence:fixture-{}", index + 1));
        }
        if item.get("subject").is_none() {
            item["subject"] = json!(RUN_ID);
        }
        if item.get("provenance").is_none() {
            item["provenance"] = json!({
                "kind": provenance_kind(item),
                "observed_by": "sidecar:dslraid-quality",
                "observed_at": "2026-06-17T00:00:00Z",
                "ontology_version": "0.1.0"
            });
        }
    }
    super::links::with_links(items);
}

fn provenance_kind(value: &Value) -> &str {
    match value.get("kind").and_then(Value::as_str) {
        Some("trace") => "runtime-trace",
        Some("coverage") => "generated",
        _ => "sidecar-assessment",
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
        "revalidate_at": "2026-07-17T00:00:00Z",
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
