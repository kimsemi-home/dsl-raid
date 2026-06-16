use super::issue::artifact_issue;
use dslraid_core::CoreIr;
use serde_json::Value;
use std::collections::BTreeMap;

pub(super) fn check_core_hash(
    ir: &CoreIr,
    lock_value: &Value,
    current_hash: &str,
    issues: &mut Vec<Value>,
) {
    let lock_core_hash = lock_value
        .get("core")
        .and_then(|core| core.get("ir_hash"))
        .and_then(Value::as_str);
    if lock_core_hash != Some(current_hash) {
        issues.push(artifact_issue(
            "ART038",
            "error",
            &format!("project:{}", ir.project.id),
            "lock core hash differs from current IR hash",
            lock_core_hash,
            Some(current_hash),
        ));
    }
}

pub(super) fn check_derivations(
    ir: &CoreIr,
    derivation_inputs: &BTreeMap<String, String>,
    current_hash: &str,
    issues: &mut Vec<Value>,
) {
    for derivation in &ir.derivations {
        match derivation_inputs.get(&derivation.id) {
            Some(input_hash) if input_hash == current_hash => {}
            Some(input_hash) => issues.push(artifact_issue(
                "ART039",
                "error",
                &derivation.id,
                "derivation input hash differs from current IR hash",
                Some(input_hash.as_str()),
                Some(current_hash),
            )),
            None => issues.push(artifact_issue(
                "ART040",
                "error",
                &derivation.id,
                "derivation is missing from lock file",
                None,
                Some("locked derivation record"),
            )),
        }
    }
}
