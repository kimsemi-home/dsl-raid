use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_requires_ssot_context() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["ssot"].as_object_mut().unwrap().remove("context");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires ssot context"]
    );
}

#[test]
fn approved_manifest_requires_ssot_ontology_version() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["ssot"]
        .as_object_mut()
        .unwrap()
        .remove("ontology_version");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires ssot ontology_version"]
    );
}

#[test]
fn approved_manifest_requires_ssot_contract_version() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["ssot"]
        .as_object_mut()
        .unwrap()
        .remove("contract_version");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires ssot contract_version"]
    );
}
