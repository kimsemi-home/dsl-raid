use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn supported_claim_rejects_control_plane_assessor() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([super::fixture::fresh(
        "medium",
        "control-plane:dslraid",
        json!(["evidence:quality"])
    )]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec![
            "supported claim claim:fresh-artifacts cannot be assessed by control plane control-plane:dslraid"
        ]
    );
}

#[test]
fn supported_claim_rejects_producer_assessor() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([super::fixture::fresh(
        "medium",
        "agent:codex",
        json!(["evidence:quality"])
    )]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec![
            "supported claim claim:fresh-artifacts cannot be self-assessed by producer agent:codex"
        ]
    );
}
