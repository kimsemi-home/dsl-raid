use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn orchestration_requires_ontology_version() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["orchestration"]
        .as_object_mut()
        .unwrap()
        .remove("ontology_version");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["orchestration receipt requires ontology_version"]
    );
}

#[test]
fn orchestration_ontology_version_must_match_ssot() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["orchestration"]["ontology_version"] = json!("9.9.9");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["orchestration ontology_version 9.9.9 differs from ssot 0.1.0"]
    );
}
