use super::super::lock;
use super::artifact_row::artifact_result;
use super::checks::{check_core_hash, check_derivations};
use super::issue::value_string;
use anyhow::Result;
use dslraid_core::{load_core_ir, sha256_json};
use serde_json::Value;
use std::path::Path;

pub(super) fn build(input: &Path, lock_path: Option<&Path>) -> Result<Value> {
    let ir = load_core_ir(input)?;
    let lock_path = lock_path
        .map(Path::to_path_buf)
        .unwrap_or_else(|| lock::inferred_lock_path(input));
    let lock_value = lock::load_lock(&lock_path)?;
    let current_ir_hash = sha256_json(&ir)?;
    let mut issues = Vec::new();

    check_core_hash(&ir, &lock_value, &current_ir_hash, &mut issues);
    check_derivations(
        &ir,
        &lock::derivation_input_map(&lock_value),
        &current_ir_hash,
        &mut issues,
    );
    let mut artifacts = ir
        .artifacts
        .iter()
        .map(|artifact| {
            artifact_result(
                &ir,
                artifact,
                &lock::artifact_map(&lock_value),
                &current_ir_hash,
                input,
                &mut issues,
            )
        })
        .collect::<Vec<_>>();

    artifacts.sort_by_key(|artifact| value_string(artifact, "artifact"));
    issues.sort_by_key(|issue| {
        format!(
            "{}:{}",
            value_string(issue, "code"),
            value_string(issue, "subject")
        )
    });
    Ok(serde_json::json!({
        "artifact_verify_version": "0.1.0",
        "status": if issues.is_empty() { "passed" } else { "failed" },
        "input": input.display().to_string(),
        "lock": lock_path.display().to_string(),
        "current_ir_hash": current_ir_hash,
        "artifacts": artifacts,
        "issues": issues
    }))
}
