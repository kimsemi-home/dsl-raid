use super::issue::artifact_issue;
use super::path::artifact_content_hash;
use dslraid_core::Artifact;
use serde_json::Value;
use std::path::Path;

pub(super) fn check_content_hash(
    artifact: &Artifact,
    record: &Value,
    input: &Path,
    status: &mut &'static str,
    issues: &mut Vec<Value>,
) {
    let expected = record.get("content_hash").and_then(Value::as_str);
    match artifact_content_hash(input, &artifact.path) {
        Some(actual) if expected == Some(actual.as_str()) => {}
        Some(actual) => push_stale(artifact, expected, Some(actual.as_str()), status, issues),
        None => push_missing(artifact, expected, status, issues),
    }
}

fn push_stale(
    artifact: &Artifact,
    expected: Option<&str>,
    actual: Option<&str>,
    status: &mut &'static str,
    issues: &mut Vec<Value>,
) {
    *status = "stale";
    issues.push(artifact_issue(
        "ART042",
        "error",
        &artifact.id,
        "artifact content hash differs from lock file",
        actual,
        expected,
    ));
}

fn push_missing(
    artifact: &Artifact,
    expected: Option<&str>,
    status: &mut &'static str,
    issues: &mut Vec<Value>,
) {
    *status = "missing";
    issues.push(artifact_issue(
        "ART040",
        "error",
        &artifact.id,
        "artifact file is missing from disk",
        None,
        expected,
    ));
}
