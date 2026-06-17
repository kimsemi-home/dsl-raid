use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_requires_producer_capability_receipt() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["producer"]
        .as_object_mut()
        .unwrap()
        .remove("reasoning_level");
    value["producer"]
        .as_object_mut()
        .unwrap()
        .remove("trust_tier");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "approved run requires producer reasoning level",
            "approved run requires producer trust tier"
        ]
    );
}

#[test]
fn approved_manifest_rejects_cold_start_producer() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["producer"]["trust_tier"] = json!("T1");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run cannot use cold-start producer agent:codex"]
    );
}

#[test]
fn automatic_authority_requires_trusted_producer() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["profile"] = json!("automatic");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["automatic authority requires trusted producer T3 or T4"]
    );
}

#[test]
fn high_risk_authority_requires_high_reasoning_producer() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["producer"]["reasoning_level"] = json!("R2");
    value["authority_gate"]["scope"] = json!("ontology");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "authority scope ontology requires human review",
            "high-risk authority requires producer reasoning level R3 or R4",
            "high-risk authority requires review capacity receipt"
        ]
    );
}
