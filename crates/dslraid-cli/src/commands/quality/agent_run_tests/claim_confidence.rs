use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn high_confidence_claim_requires_validation_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([super::claim::fixture::fresh(
        "high",
        "sidecar:dslraid-quality",
        json!(["evidence:trace"])
    )]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["high confidence claim claim:fresh-artifacts requires validation evidence"]
    );
}

#[test]
fn high_confidence_claim_requires_external_assessor() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = super::claim::fixture::fresh(
        "high",
        "sidecar:dslraid-quality",
        json!(["evidence:quality"]),
    );
    item["assessor"] = json!("agent:reviewer");
    value["claims"] = json!([item]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["high confidence claim claim:fresh-artifacts requires external assessor"]
    );
}

#[test]
fn high_confidence_claim_revalidates_degraded_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["quality"] = json!("medium");
    value["claims"] = json!([super::claim::fixture::fresh(
        "high",
        "sidecar:dslraid-quality",
        json!(["evidence:quality"])
    )]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "high confidence claim claim:fresh-artifacts requires revalidation for degraded evidence evidence:quality"
        ]
    );
}

#[test]
fn high_confidence_claim_rejects_control_plane_assessor() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = super::claim::fixture::fresh(
        "high",
        "sidecar:dslraid-quality",
        json!(["evidence:quality"]),
    );
    item["assessor"] = json!("control-plane:dslraid");
    value["claims"] = json!([item]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "high confidence claim claim:fresh-artifacts cannot be assessed by control plane control-plane:dslraid"
        ]
    );
}
