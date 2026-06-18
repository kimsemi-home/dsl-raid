use super::super::fixtures::{base_manifest, fresh_lock, high_snapshot};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(super) fn issues(root: &Path, coverage: &str, trace: &str) -> Vec<String> {
    super::super::super::agent_run::semantic_issues_with_context(
        &manifest(coverage, trace),
        &fresh_lock(),
        root,
    )
}

pub(super) fn temp_root(name: &str) -> PathBuf {
    let path =
        std::env::temp_dir().join(format!("dslraid-agent-run-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

pub(super) fn write_trace(root: &Path, path: &str) {
    write_json(
        root,
        path,
        json!({"trace_version":"0.1.0","run":{"id":"run:one"},"events":[]}),
    );
}

pub(super) fn write_coverage(root: &Path, path: &str, design: &str, trace: &str) {
    write_json(
        root,
        path,
        json!({
            "coverage_version":"0.1.0",
            "design_ir":{"path":design},
            "traces":[{"path":trace}],
            "subjects":[]
        }),
    );
}

fn manifest(coverage: &str, trace: &str) -> Value {
    base_manifest(
        json!([{ "id": "reviewer:quality" }]),
        "finished",
        json!([
            { "quality": "high", "kind": "validation", "quality_snapshots": high_snapshot() },
            { "quality": "high", "kind": "trace", "uri": trace, "quality_snapshots": high_snapshot() },
            { "quality": "high", "kind": "coverage", "uri": coverage, "quality_snapshots": high_snapshot() }
        ]),
    )
}

fn write_json(root: &Path, path: &str, value: Value) {
    fs::write(root.join(path), serde_json::to_vec(&value).unwrap()).unwrap();
}
