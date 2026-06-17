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
    json!([
        { "quality": "high", "kind": "validation" },
        { "quality": "high", "kind": "trace" },
        { "quality": "high", "kind": "coverage" }
    ])
}

pub(super) fn fresh_lock() -> Value {
    json!({
        "core": { "ir_hash": "sha256:core" },
        "artifacts": [
            { "path": "generated/runtime_fsm.rs", "status": "fresh" }
        ]
    })
}
