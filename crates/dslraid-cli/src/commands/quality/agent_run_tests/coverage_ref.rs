use super::fixtures::{base_manifest, fresh_lock};
use serde_json::json;
use std::path::PathBuf;

#[test]
fn approved_manifest_rejects_missing_coverage_file() {
    let value = manifest_with_coverage("missing/run.coverage.json");

    assert_eq!(
        super::super::agent_run::semantic_issues_with_context(&value, &fresh_lock(), &repo_root()),
        vec!["coverage evidence missing/run.coverage.json does not exist"]
    );
}

#[test]
fn approved_manifest_rejects_invalid_coverage_schema() {
    let value = manifest_with_coverage("examples/runscope/runscope.raid.json");

    assert_eq!(
        super::super::agent_run::semantic_issues_with_context(&value, &fresh_lock(), &repo_root()),
        vec!["coverage evidence examples/runscope/runscope.raid.json failed coverage schema"]
    );
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn manifest_with_coverage(uri: &str) -> serde_json::Value {
    base_manifest(
        json!([{ "id": "reviewer:quality" }]),
        "finished",
        json!([
            { "quality": "high", "kind": "validation" },
            { "quality": "high", "kind": "trace", "uri": "examples/runscope/run-001.trace.json" },
            { "quality": "high", "kind": "coverage", "uri": uri }
        ]),
    )
}
