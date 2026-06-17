use super::fixtures::{base_manifest, high_snapshot};
use serde_json::json;

#[test]
fn approved_manifest_rejects_missing_trace_evidence() {
    let value = base_manifest(
        json!([{ "id": "reviewer:quality" }]),
        "finished",
        json!([
            { "quality": "high", "kind": "validation", "quality_snapshots": high_snapshot() },
            { "quality": "high", "kind": "coverage", "quality_snapshots": high_snapshot() }
        ]),
    );

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires trace evidence"]
    );
}

#[test]
fn approved_manifest_rejects_missing_coverage_evidence() {
    let value = base_manifest(
        json!([{ "id": "reviewer:quality" }]),
        "finished",
        json!([
            { "quality": "high", "kind": "validation", "quality_snapshots": high_snapshot() },
            { "quality": "high", "kind": "trace", "quality_snapshots": high_snapshot() }
        ]),
    );

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires coverage evidence"]
    );
}
