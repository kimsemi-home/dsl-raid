use super::super::super::fields::{field_is, field_text, items, text};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) type TraceUris = BTreeSet<String>;

pub(super) fn push_issues(
    manifest: &Value,
    coverage: &Value,
    uri: &str,
    traces: &TraceUris,
    issues: &mut Vec<String>,
) {
    push_design_issue(manifest, coverage, uri, issues);
    push_trace_issue(coverage, traces, uri, issues);
}

pub(super) fn trace_uris(value: &Value) -> TraceUris {
    items(value, "evidence")
        .filter(|item| field_is(item, "kind", "trace"))
        .filter_map(|item| field_text(item, "uri").map(str::to_string))
        .collect()
}

fn push_design_issue(manifest: &Value, coverage: &Value, uri: &str, issues: &mut Vec<String>) {
    if text(manifest, &["ssot", "core_ir"]) != text(coverage, &["design_ir", "path"]) {
        issues.push(format!(
            "coverage evidence {uri} design_ir does not match manifest ssot"
        ));
    }
}

fn push_trace_issue(coverage: &Value, traces: &TraceUris, uri: &str, issues: &mut Vec<String>) {
    let covered = items(coverage, "traces")
        .filter_map(|item| field_text(item, "path"))
        .any(|path| traces.contains(path));
    if !covered {
        issues.push(format!(
            "coverage evidence {uri} must reference trace evidence"
        ));
    }
}
