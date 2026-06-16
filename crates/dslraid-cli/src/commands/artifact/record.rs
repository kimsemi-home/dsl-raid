use super::issue::artifact_issue;
use super::record_status::check_record_status;
use dslraid_core::Artifact;
use serde_json::Value;

pub(super) fn check_lock_record(
    artifact: &Artifact,
    record: &Value,
    current_hash: &str,
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

fn check_input_hash(
    artifact: &Artifact,
    record: &Value,
    current_hash: &str,
    status: &mut &'static str,
    issues: &mut Vec<Value>,
) {
    let input_hash = record.get("input_hash").and_then(Value::as_str);
    if input_hash == Some(current_hash) {
        return;
    }
    *status = "stale";
    issues.push(artifact_issue(
        "ART039",
        "error",
        &artifact.id,
        "artifact input hash differs from current IR hash",
        input_hash,
        Some(current_hash),
    ));
}
