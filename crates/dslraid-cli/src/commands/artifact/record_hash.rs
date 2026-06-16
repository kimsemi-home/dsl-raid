use super::issue::artifact_issue;
use dslraid_core::Artifact;
use serde_json::Value;

pub(super) fn check_input_hash(
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
