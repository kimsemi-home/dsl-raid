use super::issue::artifact_issue;
use super::record::check_lock_record;
use dslraid_core::{Artifact, CoreIr};
use serde_json::Value;
use std::collections::BTreeMap;

pub(super) fn artifact_result(
    ir: &CoreIr,
    artifact: &Artifact,
    lock_artifacts: &BTreeMap<String, Value>,
    current_hash: &str,
    issues: &mut Vec<Value>,
) -> Value {
    let mut status = "fresh";
    check_derivation_link(ir, artifact, &mut status, issues);
    match lock_artifacts.get(&artifact.id) {
        Some(record) => check_lock_record(artifact, record, current_hash, &mut status, issues),
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

fn check_derivation_link(
    ir: &CoreIr,
    artifact: &Artifact,
    status: &mut &'static str,
    issues: &mut Vec<Value>,
) {
    if !requires_lock_record(artifact) {
        return;
    }
    match artifact.generated_by.as_deref() {
        Some(derivation) if ir.derivation_by_id(derivation).is_some() => {}
        Some(derivation) => {
            *status = "stale";
            issues.push(artifact_issue(
                "ART034",
                "error",
                &artifact.id,
                "artifact references an unknown derivation",
                Some(derivation),
                Some("existing derivation"),
            ));
        }
        None => {
            *status = "stale";
            issues.push(artifact_issue(
                "ART034",
                "error",
                &artifact.id,
                "generated artifact has no generated_by derivation",
                None,
                Some("generated_by"),
            ));
        }
    }
}

fn requires_lock_record(artifact: &Artifact) -> bool {
    artifact.kind == "generated" || artifact.kind == "test" || artifact.kind == "doc"
}
