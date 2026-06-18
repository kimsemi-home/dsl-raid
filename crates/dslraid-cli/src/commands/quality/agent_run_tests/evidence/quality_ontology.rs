use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn high_quality_snapshot_requires_matching_ontology() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["quality_snapshots"] = json!([{
        "assessor": "sidecar:dslraid-quality",
        "quality": "high",
        "revalidate_at": "2026-07-17T00:00:00Z",
        "ontology_version": "9.9.9"
    }]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec![
            "evidence evidence:quality high quality snapshot ontology 9.9.9 differs from ssot 0.1.0"
        ]
    );
}

#[test]
fn high_quality_snapshot_requires_ontology_version() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["quality_snapshots"] = json!([{
        "assessor": "sidecar:dslraid-quality",
        "quality": "high",
        "revalidate_at": "2026-07-17T00:00:00Z"
    }]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["evidence evidence:quality high quality snapshot requires ontology_version"]
    );
}
