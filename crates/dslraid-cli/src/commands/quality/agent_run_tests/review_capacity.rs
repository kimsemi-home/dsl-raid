use super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn high_risk_authority_requires_review_capacity() {
    let mut value = high_risk_manifest();
    value.as_object_mut().unwrap().remove("review_capacity");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["high-risk authority requires review capacity receipt"]
    );
}

#[test]
fn frozen_capacity_blocks_high_risk_sidecar_authority() {
    let mut value = high_risk_manifest();
    value["review_capacity"] = capacity("frozen", 5, 3, json!(["evidence:quality"]));

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "review capacity queue depth exceeds max",
            "review capacity frozen freezes high-risk sidecar authority"
        ]
    );
}

#[test]
fn governance_authority_survives_frozen_capacity() {
    let mut value = high_risk_manifest();
    value["authority_gate"]["profile"] = json!("governance");
    value["orchestration"]["authority_profile"] = json!("governance");
    value["authority_gate"]["approved_by"] = json!("steward:ops");
    value["review_capacity"] = capacity("frozen", 5, 5, json!(["evidence:quality"]));

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

#[test]
fn review_capacity_rejects_unknown_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["review_capacity"] = capacity("available", 1, 5, json!(["evidence:missing"]));

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["review capacity references unknown evidence evidence:missing"]
    );
}

fn high_risk_manifest() -> Value {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["authority_gate"]["scope"] = json!("ontology");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("human:alice");
    value["review_capacity"] = capacity("available", 1, 5, json!(["evidence:quality"]));
    value
}

fn capacity(status: &str, depth: u64, max: u64, evidence: Value) -> Value {
    json!({
        "status": status,
        "queue_depth": depth,
        "max_queue_depth": max,
        "assessed_at": "2026-06-17T00:00:00Z",
        "assessor": "sidecar:dslraid-quality",
        "evidence": evidence
    })
}
