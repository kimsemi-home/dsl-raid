use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn orchestration_requires_shadow_receipt() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["orchestration"]
        .as_object_mut()
        .unwrap()
        .remove("shadow");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["orchestration receipt requires shadow"]
    );
}

#[test]
fn shadow_orchestrator_must_be_independent() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["orchestration"]["shadow"]["routed_by"] = json!("control-plane:dslraid");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["shadow orchestrator cannot be primary control-plane:dslraid"]
    );
}

#[test]
fn severe_shadow_difference_requires_human_review() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["orchestration"]["shadow"]["severity"] = json!("D3");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["shadow severity D3 requires human review"]
    );
}

#[test]
fn shadow_evidence_must_exist() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["orchestration"]["shadow"]["evidence"] = json!(["evidence:missing"]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["shadow orchestration references unknown evidence evidence:missing"]
    );
}
