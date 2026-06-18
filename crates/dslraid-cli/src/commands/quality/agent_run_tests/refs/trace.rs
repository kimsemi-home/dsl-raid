use super::super::fixtures::{base_manifest, fresh_lock, high_snapshot};
use serde_json::json;
use std::path::PathBuf;

#[test]
fn approved_manifest_rejects_missing_trace_file() {
    let value = manifest_with_trace("missing/run.trace.json");

    assert_eq!(
        super::super::super::agent_run::semantic_issues_with_context(
            &value,
            &fresh_lock(),
            &repo_root(),
        ),
        vec!["trace evidence missing/run.trace.json does not exist"]
    );
}

#[test]
fn approved_manifest_rejects_invalid_trace_schema() {
    let value = manifest_with_trace("examples/runscope/runscope.raid.json");

    assert_eq!(
        super::super::super::agent_run::semantic_issues_with_context(
            &value,
            &fresh_lock(),
            &repo_root(),
        ),
        vec!["trace evidence examples/runscope/runscope.raid.json failed trace schema"]
    );
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn manifest_with_trace(uri: &str) -> serde_json::Value {
    base_manifest(
        json!([{ "id": "reviewer:quality" }]),
        "finished",
        json!([
            { "quality": "high", "kind": "validation", "quality_snapshots": high_snapshot() },
            { "quality": "high", "kind": "trace", "uri": uri, "quality_snapshots": high_snapshot() },
            { "quality": "high", "kind": "coverage", "quality_snapshots": high_snapshot() }
        ]),
    )
}
