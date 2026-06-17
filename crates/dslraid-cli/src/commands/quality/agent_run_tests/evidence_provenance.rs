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
            "evidence evidence:quality requires provenance observed_at"
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
