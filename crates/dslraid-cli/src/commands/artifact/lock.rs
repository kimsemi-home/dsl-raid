use crate::validate_json_file;
use anyhow::Result;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn load_lock(path: &Path) -> Result<Value> {
    validate_json_file(
        &repo_relative_path("schemas/dslraid-lock.schema.json"),
        path,
    )?;
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

pub(super) fn inferred_lock_path(input: &Path) -> PathBuf {
    let Some(file_name) = input.file_name().and_then(|name| name.to_str()) else {
        return input.with_extension("lock.json");
    };
    let lock_name = if let Some(prefix) = file_name.strip_suffix(".raid.json") {
        format!("{prefix}.lock.json")
    } else if let Some(prefix) = file_name.strip_suffix(".dslraid.json") {
        format!("{prefix}.dslraid.lock.json")
    } else if let Some(prefix) = file_name.strip_suffix(".json") {
        format!("{prefix}.lock.json")
    } else {
        format!("{file_name}.lock.json")
    };
    input
        .parent()
        .map(|parent| parent.join(&lock_name))
        .unwrap_or_else(|| PathBuf::from(lock_name))
}

pub(super) fn artifact_map(lock_value: &Value) -> BTreeMap<String, Value> {
    lock_value
        .get("artifacts")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|artifact| {
            artifact
                .get("artifact")
                .and_then(Value::as_str)
                .map(|id| (id.to_string(), artifact.clone()))
        })
        .collect()
}

pub(super) fn derivation_input_map(lock_value: &Value) -> BTreeMap<String, String> {
    lock_value
        .get("derivations")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|derivation| {
            let id = derivation.get("derivation").and_then(Value::as_str)?;
            let input_hash = derivation.get("input_hash").and_then(Value::as_str)?;
            Some((id.to_string(), input_hash.to_string()))
        })
        .collect()
}

fn repo_relative_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.exists() {
        return direct;
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}
