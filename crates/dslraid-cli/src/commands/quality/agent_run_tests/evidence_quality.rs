use super::fixtures::{base_manifest, high, high_snapshot};
use serde_json::json;

#[test]
fn approved_manifest_rejects_high_evidence_without_snapshot() {
    let value = base_manifest(
        json!([{ "id": "reviewer:quality" }]),
        "finished",
        json!([
            { "id": "evidence:quality", "quality": "high", "kind": "validation" },
            { "id": "evidence:trace", "quality": "high", "kind": "trace" },
            { "id": "evidence:coverage", "quality": "high", "kind": "coverage" }
        ]),
    );

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "evidence evidence:quality requires high quality snapshot",
            "evidence evidence:trace requires high quality snapshot",
            "evidence evidence:coverage requires high quality snapshot",
            "approved run requires high quality evidence snapshot"
        ]
    );
}

#[test]
fn approved_manifest_rejects_self_assessed_quality_snapshot() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["evidence"][0]["quality_snapshots"] = json!([{
        "assessed_at": "2026-06-17T00:00:00Z",
        "assessor": "agent:codex",
        "purpose": "authority",
        "quality": "high",
        "revalidate_at": "2026-07-17T00:00:00Z",
        "ontology_version": "0.1.0"
    }]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["evidence evidence:quality quality snapshot must be independent"]
    );
}

#[test]
fn approved_manifest_accepts_sidecar_quality_snapshot() {
    let value = base_manifest(
        json!([{ "id": "reviewer:quality" }]),
        "finished",
        json!([
            { "quality": "high", "kind": "validation", "quality_snapshots": high_snapshot() },
            { "quality": "high", "kind": "trace", "quality_snapshots": high_snapshot() },
            { "quality": "high", "kind": "coverage", "quality_snapshots": high_snapshot() }
        ]),
    );

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}
