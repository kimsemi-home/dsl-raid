use super::super::kind::requires_lock_record;
use super::derivation::check_derivation_link;
use super::issue::artifact_issue;
use super::record::check_lock_record;
use dslraid_core::{Artifact, CoreIr};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

pub(super) fn artifact_result(
    ir: &CoreIr,
    artifact: &Artifact,
    lock_artifacts: &BTreeMap<String, Value>,
    current_hash: &str,
    input: &Path,
    issues: &mut Vec<Value>,
) -> Value {
    let mut status = "fresh";
    check_derivation_link(ir, artifact, &mut status, issues);
    match lock_artifacts.get(&artifact.id) {
        Some(record) => {
            check_lock_record(artifact, record, current_hash, input, &mut status, issues)
        }
        None if requires_lock_record(artifact) => {
            status = "missing";
            issues.push(artifact_issue(
                "ART040",
                "error",
                &artifact.id,
                "artifact is missing from lock file",
                None,
                Some("locked artifact record"),
            ));
        }
        None => status = "external",
    }
    serde_json::json!({
        "artifact": artifact.id,
        "path": artifact.path,
        "kind": artifact.kind,
        "generated_by": artifact.generated_by,
        "status": status
    })
}
