pub(super) mod fixture;

use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_rejects_supported_claim_without_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([fixture::fresh(
        "medium",
        "sidecar:dslraid-quality",
        json!([])
    )]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["supported claim claim:fresh-artifacts requires evidence"]
    );
}

#[test]
fn approved_manifest_rejects_unknown_claim_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([fixture::fresh(
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
    value["claims"] = json!([fixture::fresh(
        "high",
        "agent:codex",
        json!(["evidence:quality"])
    )]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["high confidence claim claim:fresh-artifacts cannot be self-assessed"]
    );
}

#[test]
fn approved_manifest_rejects_claim_without_interpreter() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = fixture::fresh(
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
