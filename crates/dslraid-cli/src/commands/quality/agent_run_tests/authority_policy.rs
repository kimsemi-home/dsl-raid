use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_requires_policy_hash() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]
        .as_object_mut()
        .unwrap()
        .remove("policy_hash");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved authority gate requires policy hash"]
    );
}

#[test]
fn approved_manifest_requires_human_review_flag() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]
        .as_object_mut()
        .unwrap()
        .remove("human_review_required");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved authority gate requires human review flag"]
    );
}

#[test]
fn approved_manifest_requires_approver() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]
        .as_object_mut()
        .unwrap()
        .remove("approved_by");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved authority gate requires approver"]
    );
}

#[test]
fn approved_manifest_rejects_mutable_policy_pointer() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["policy_hash"] = json!("policy:latest");
    value["orchestration"]["policy_hash"] = json!("policy:latest");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved authority gate cannot use mutable policy pointer policy:latest"]
    );
}
