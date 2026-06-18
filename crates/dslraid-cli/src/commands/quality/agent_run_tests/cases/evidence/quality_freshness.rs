use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn high_quality_snapshot_requires_revalidation_deadline() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["quality_snapshots"] = json!([{
        "assessed_at": "2026-06-17T00:00:00Z",
        "assessor": "sidecar:dslraid-quality",
        "purpose": "authority",
        "quality": "high",
        "ontology_version": "0.1.0"
    }]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["evidence evidence:quality high quality snapshot requires revalidate_at"]
    );
}
