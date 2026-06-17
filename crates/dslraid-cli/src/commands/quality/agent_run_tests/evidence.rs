use super::fixtures::base_manifest;
use serde_json::json;

#[test]
fn approved_manifest_rejects_missing_trace_evidence() {
    let value = base_manifest(
        json!([{ "id": "reviewer:quality" }]),
        "finished",
        json!([
            { "quality": "high", "kind": "validation" },
            { "quality": "high", "kind": "coverage" }
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
            { "quality": "high", "kind": "validation" },
            { "quality": "high", "kind": "trace" }
        ]),
    );

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires coverage evidence"]
    );
}
