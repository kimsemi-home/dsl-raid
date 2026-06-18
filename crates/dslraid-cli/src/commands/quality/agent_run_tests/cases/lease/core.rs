use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_requires_lease_ontology_version() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["lease"]
        .as_object_mut()
        .unwrap()
        .remove("ontology_version");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires lease ontology_version"]
    );
}

#[test]
fn lease_ontology_version_must_match_ssot() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["lease"]["ontology_version"] = json!("9.9.9");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["lease ontology_version differs from ssot ontology"]
    );
}
