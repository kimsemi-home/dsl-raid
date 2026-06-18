use super::content_hash::check_content_hash;
use super::issue::artifact_issue;
use super::record_hash::check_input_hash;
use super::record_status::check_record_status;
use dslraid_core::Artifact;
use serde_json::Value;
use std::path::Path;

pub(super) fn check_lock_record(
    artifact: &Artifact,
    record: &Value,
    current_hash: &str,
    input: &Path,
    status: &mut &'static str,
    issues: &mut Vec<Value>,
) {
    check_field(
        artifact,
        "path",
        record.get("path").and_then(Value::as_str),
        Some(artifact.path.as_str()),
        status,
        issues,
    );
    check_field(
        artifact,
        "kind",
        record.get("kind").and_then(Value::as_str),
        Some(artifact.kind.as_str()),
        status,
        issues,
    );
    check_field(
        artifact,
        "generated_by",
        record.get("generated_by").and_then(Value::as_str),
        artifact.generated_by.as_deref(),
        status,
        issues,
    );
    check_input_hash(artifact, record, current_hash, status, issues);
    check_content_hash(artifact, record, input, status, issues);
    check_record_status(artifact, record, status, issues);
}

fn check_field(
    artifact: &Artifact,
    field: &str,
    actual: Option<&str>,
    expected: Option<&str>,
    status: &mut &'static str,
    issues: &mut Vec<Value>,
) {
    if actual == expected {
        return;
    }
    *status = "stale";
    issues.push(artifact_issue(
        "ART041",
        "error",
        &artifact.id,
        &format!("artifact {field} differs from lock file"),
        actual,
        expected,
    ));
}
