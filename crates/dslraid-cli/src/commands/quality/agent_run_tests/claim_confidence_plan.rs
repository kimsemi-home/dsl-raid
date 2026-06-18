use super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn high_confidence_claim_requires_verification_plan() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = claim(json!(["evidence:quality"]));
    item.as_object_mut().unwrap().remove("verification_plan");
    value["claims"] = json!([item]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["high confidence claim claim:fresh-artifacts requires verification plan"]
    );
}

fn claim(evidence: Value) -> Value {
    json!({
        "id": "claim:fresh-artifacts",
        "subject": "agent-run:runscope-quality-001",
        "statement": "Fresh conformance matches the canonical IR.",
        "confidence": "high",
        "assessor": "sidecar:dslraid-quality",
        "interpreted_under": "0.1.0",
        "verification_plan": "verification:quality",
        "status": "supported",
        "evidence": evidence
    })
}
