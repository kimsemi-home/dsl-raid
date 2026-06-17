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
