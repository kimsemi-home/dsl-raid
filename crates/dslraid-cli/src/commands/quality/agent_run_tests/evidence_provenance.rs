mod bridge;

use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn evidence_requires_provenance_fields() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]
        .as_object_mut()
        .unwrap()
        .remove("provenance");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "evidence evidence:quality requires provenance kind",
            "evidence evidence:quality requires provenance observed_by",
            "evidence evidence:quality requires provenance observed_at",
            "evidence evidence:quality requires provenance ontology_version"
        ]
    );
}

#[test]
fn evidence_rejects_unknown_provenance_kind() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["provenance"]["kind"] = json!("rumor");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["evidence evidence:quality has unsupported provenance kind rumor"]
    );
}

#[test]
fn evidence_provenance_ontology_must_match_ssot() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["provenance"]["ontology_version"] = json!("9.9.9");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["evidence evidence:quality provenance ontology 9.9.9 requires translation bridge to ssot 0.1.0"]
    );
}
