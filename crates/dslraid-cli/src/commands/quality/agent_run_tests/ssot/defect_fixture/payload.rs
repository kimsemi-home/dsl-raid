use serde_json::{json, Value};

pub(super) fn claim(plan: Option<&str>) -> Value {
    let mut value = json!({
        "id": "claim:ssot-defect",
        "subject": "agent-run:runscope-quality-001",
        "statement": "SSOT Defect is confirmed in the canonical IR.",
        "confidence": "medium",
        "assessor": "sidecar:dslraid-quality",
        "interpreted_under": "0.1.0",
        "status": "supported",
        "evidence": ["evidence:quality"]
    });
    if let Some(plan) = plan {
        value["verification_plan"] = json!(plan);
    }
    value
}

pub(super) fn semantic_diff() -> Value {
    json!({
        "id": "semantic-diff:ssot-defect",
        "base_hash": "sha256:base",
        "head_hash": "sha256:core",
        "status": "changed",
        "summary": "Canonical IR changed to patch the SSOT defect.",
        "evidence": ["evidence:quality", "evidence:trace"]
    })
}

pub(super) fn quarantine() -> Value {
    json!({
        "id": "containment:ssot-defect",
        "kind": "quarantine",
        "subject": "agent-run:runscope-quality-001",
        "reason": "Freeze affected SSOT scope during patch validation.",
        "status": "released",
        "owner": "steward:ops",
        "opened_at": "2026-06-17T00:00:00Z",
        "released_by": "steward:ops",
        "evidence": ["evidence:quality"],
        "release_conditions": [release_condition()]
    })
}

fn release_condition() -> Value {
    json!({
        "id": "release:ssot-defect-verified",
        "description": "Conformance verification passed.",
        "status": "met",
        "evidence": ["evidence:quality"]
    })
}

pub(super) fn capacity() -> Value {
    json!({
        "status": "available",
        "queue_depth": 1,
        "max_queue_depth": 5,
        "assessed_at": "2026-06-17T00:00:00Z",
        "assessor": "sidecar:dslraid-quality",
        "evidence": ["evidence:quality"]
    })
}
