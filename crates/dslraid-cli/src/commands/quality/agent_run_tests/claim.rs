use super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn approved_manifest_rejects_supported_claim_without_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([claim("medium", "sidecar:dslraid-quality", json!([]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["supported claim claim:fresh-artifacts requires evidence"]
    );
}

#[test]
fn approved_manifest_rejects_unknown_claim_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([claim(
        "medium",
        "sidecar:dslraid-quality",
        json!(["evidence:missing"])
    )]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["claim claim:fresh-artifacts references unknown evidence evidence:missing"]
    );
}

#[test]
fn approved_manifest_rejects_self_assessed_high_confidence_claim() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([claim("high", "agent:codex", json!(["evidence:quality"]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["high confidence claim claim:fresh-artifacts cannot be self-assessed"]
    );
}

#[test]
fn approved_manifest_rejects_claim_without_interpreter() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = claim(
        "high",
        "sidecar:dslraid-quality",
        json!(["evidence:quality"]),
    );
    item.as_object_mut().unwrap().remove("interpreted_under");
    value["claims"] = json!([item]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["claim claim:fresh-artifacts requires interpreted_under"]
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
