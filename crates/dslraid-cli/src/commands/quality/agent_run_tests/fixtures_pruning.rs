use serde_json::{json, Value};

pub(super) fn push_pruned_extra(value: &mut Value) {
    value["evidence"].as_array_mut().unwrap().push(json!({
        "id": "evidence:old-validation",
        "kind": "validation",
        "uri": "evidence/old-validation.json",
        "status": "pruned",
        "subject": "agent-run:runscope-quality-001",
        "provenance": {
            "kind": "sidecar-assessment",
            "observed_by": "sidecar:dslraid-quality",
            "observed_at": "2026-06-17T00:00:00Z",
            "ontology_version": "0.1.0"
        }
    }));
}

pub(super) fn tombstone() -> Value {
    json!({
        "reason": "superseded by newer validation",
        "pruned_by": "sidecar:dslraid-quality",
        "pruned_at": "2026-06-18T00:00:00Z",
        "policy_hash": "sha256:policy"
    })
}
