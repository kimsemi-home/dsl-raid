use super::super::kind::requires_lock_record;
use super::issue::artifact_issue;
use dslraid_core::{Artifact, CoreIr};
use serde_json::Value;

pub(super) fn check_derivation_link(
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
