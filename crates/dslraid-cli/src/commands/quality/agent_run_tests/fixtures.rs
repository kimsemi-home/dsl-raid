use serde_json::{json, Value};

pub(super) fn base_manifest(reviewers: Value, lease: &str, evidence: Value) -> Value {
    json!({
        "run": { "status": "verified" },
        "ssot": {
            "core_ir": "examples/runscope/runscope.raid.json",
            "core_ir_hash": "sha256:core"
        },
        "producer": { "id": "agent:codex" },
        "reviewers": reviewers,
        "authority_gate": {
            "decision": "approved",
            "approved_by": "gate:quality"
        },
        "lease": { "status": lease },
        "evidence": evidence,
        "artifacts": [],
        "debts": []
    })
}

pub(super) fn high() -> Value {
    let snapshot = high_snapshot();
    json!([
        { "quality": "high", "kind": "validation", "quality_snapshots": snapshot.clone() },
        { "quality": "high", "kind": "trace", "quality_snapshots": snapshot.clone() },
        { "quality": "high", "kind": "coverage", "quality_snapshots": snapshot }
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
