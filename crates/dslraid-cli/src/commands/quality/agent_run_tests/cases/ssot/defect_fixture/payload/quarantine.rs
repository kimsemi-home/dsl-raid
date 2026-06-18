use serde_json::{json, Value};

pub(super) fn fixture() -> Value {
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
