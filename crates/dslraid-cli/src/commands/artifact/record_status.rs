use super::issue::artifact_issue;
use dslraid_core::Artifact;
use serde_json::Value;

pub(super) fn check_record_status(
    artifact: &Artifact,
    record: &Value,
    status: &mut &'static str,
    issues: &mut Vec<Value>,
) {
    match record.get("status").and_then(Value::as_str) {
        Some("stale") => push_status_issue(artifact, "stale", "fresh", status, issues),
        Some("missing") => push_status_issue(artifact, "missing", "fresh", status, issues),
        _ => {}
    }
}

fn push_status_issue(
    artifact: &Artifact,
    actual: &'static str,
    expected: &'static str,
    status: &mut &'static str,
    issues: &mut Vec<Value>,
) {
    *status = actual;
    issues.push(artifact_issue(
        if actual == "missing" {
            "ART040"
        } else {
            "ART039"
        },
        "error",
        &artifact.id,
        &format!("lock file marks artifact as {actual}"),
        Some(actual),
        Some(expected),
    ));
}
