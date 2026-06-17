use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn authority_gate_rejects_pruned_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["status"] = json!("pruned");
    value["evidence"][0]["tombstone"] = tombstone();

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "authority gate references pruned evidence evidence:quality",
            "approved authority gate requires validation or decision evidence"
        ]
    );
}

fn tombstone() -> serde_json::Value {
    json!({
        "reason": "superseded by newer validation",
        "pruned_by": "sidecar:dslraid-quality",
        "pruned_at": "2026-06-18T00:00:00Z",
        "policy_hash": "sha256:policy"
    })
}
