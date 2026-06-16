use anyhow::{Context, Result};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

use crate::CoreIr;

pub fn load_core_ir(path: impl AsRef<Path>) -> Result<CoreIr> {
    let path = path.as_ref();
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse core IR {}", path.display()))
}

pub fn load_json_value(path: impl AsRef<Path>) -> Result<Value> {
    let path = path.as_ref();
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse JSON {}", path.display()))
}

pub fn canonical_json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    serde_json::to_vec_pretty(value).context("failed to serialize canonical JSON")
}

pub fn repo_schema_path(repo_root: impl AsRef<Path>, schema: &str) -> PathBuf {
    repo_root.as_ref().join(schema)
}
