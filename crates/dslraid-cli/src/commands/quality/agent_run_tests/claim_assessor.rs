use super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn supported_claim_rejects_control_plane_assessor() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([claim(
        "medium",
        "control-plane:dslraid",
        json!(["evidence:quality"])
    )]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "supported claim claim:fresh-artifacts cannot be assessed by control plane control-plane:dslraid"
        ]
    );
}

fn claim(confidence: &str, assessor: &str, evidence: Value) -> Value {
    json!({
        "id": "claim:fresh-artifacts",
        "subject": "agent-run:runscope-quality-001",
        "statement": "Fresh conformance matches the canonical IR.",
        "confidence": confidence,
        "assessor": assessor,
        "interpreted_under": "0.1.0",
        "status": "supported",
        "evidence": evidence
    })
}
