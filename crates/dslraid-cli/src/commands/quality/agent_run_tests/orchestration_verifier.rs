use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn orchestration_requires_control_plane_verifier() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["orchestration"]
        .as_object_mut()
        .unwrap()
        .remove("verified_by");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["orchestration receipt requires verified_by"]
    );
}

#[test]
fn control_plane_verifier_must_be_independent_reviewer() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["orchestration"]["routed_by"] = json!("reviewer:quality");
    value["orchestration"]["verified_by"] = json!("agent:codex");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "control plane verifier cannot be selected producer agent:codex",
            "control plane verifier cannot be manifest producer agent:codex",
            "control plane verifier agent:codex must be a reviewer"
        ]
    );
}

#[test]
fn control_plane_verifier_cannot_be_orchestrator() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["orchestration"]["verified_by"] = json!("control-plane:dslraid");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "control plane verifier cannot be orchestrator control-plane:dslraid",
            "control plane verifier control-plane:dslraid must be a reviewer"
        ]
    );
}
