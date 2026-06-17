use super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn high_confidence_claim_requires_validation_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([claim(json!(["evidence:trace"]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["high confidence claim claim:fresh-artifacts requires validation evidence"]
    );
}

#[test]
fn high_confidence_claim_requires_external_assessor() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = claim(json!(["evidence:quality"]));
    item["assessor"] = json!("agent:reviewer");
    value["claims"] = json!([item]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["high confidence claim claim:fresh-artifacts requires external assessor"]
    );
}

fn claim(evidence: Value) -> Value {
    json!({
        "id": "claim:fresh-artifacts",
        "subject": "agent-run:runscope-quality-001",
        "statement": "Generated artifacts match the canonical IR.",
        "confidence": "high",
        "assessor": "sidecar:dslraid-quality",
        "interpreted_under": "0.1.0",
        "status": "supported",
        "evidence": evidence
    })
}
