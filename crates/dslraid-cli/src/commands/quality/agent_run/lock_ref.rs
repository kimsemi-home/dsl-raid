use super::fields::{field_text, items, text};
use serde_json::Value;
use std::collections::BTreeMap;

pub(super) fn push_issues(value: &Value, lock: Option<&Value>, issues: &mut Vec<String>) {
    let Some(lock) = lock else {
        return;
    };
    push_core_hash_issue(value, lock, issues);
    let lock = lock_artifacts(lock);
    for artifact in
        items(value, "artifacts").filter(|item| field_text(item, "status") == Some("verified"))
    {
        push_artifact_issue(artifact, &lock, issues);
    }
}

fn push_core_hash_issue(value: &Value, lock: &Value, issues: &mut Vec<String>) {
    let manifest_hash = text(value, &["ssot", "core_ir_hash"]);
    if manifest_hash.is_some() && manifest_hash != text(lock, &["core", "ir_hash"]) {
        issues.push("manifest core_ir_hash differs from lock core hash".to_string());
    }
}

fn push_artifact_issue(
    artifact: &Value,
    lock: &BTreeMap<String, String>,
    issues: &mut Vec<String>,
) {
    let Some(path) = field_text(artifact, "path") else {
        return;
    };
    if lock.get(path).map(String::as_str) != Some("fresh") {
        issues.push(format!("verified artifact {path} must be fresh in lock"));
    }
}

fn lock_artifacts(lock: &Value) -> BTreeMap<String, String> {
    items(lock, "artifacts")
        .filter_map(|item| {
            Some((
                field_text(item, "path")?.to_string(),
                field_text(item, "status")?.to_string(),
            ))
        })
        .collect()
}
