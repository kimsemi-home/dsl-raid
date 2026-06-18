mod domains;
mod fields;

use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::{fs, path::Path};

pub(super) fn check(path: &Path, lock_path: &Path) -> Result<()> {
    let value = read_json(path)?;
    let lock = read_json(lock_path)?;
    let issues = semantic_issues_with_context(&value, &lock, Path::new("."));
    if issues.is_empty() {
        println!("agent run semantic gate ok");
        return Ok(());
    }
    bail!("agent run semantic gate failed: {}", issues.join("; "))
}

fn read_json(path: &Path) -> Result<Value> {
    serde_json::from_slice(&fs::read(path).with_context(|| format!("read {}", path.display()))?)
        .with_context(|| format!("parse {}", path.display()))
}

#[cfg(test)]
pub(super) fn semantic_issues(value: &Value) -> Vec<String> {
    semantic_issues_with_optional_context(value, None, None)
}

pub(super) fn semantic_issues_with_context(
    value: &Value,
    lock: &Value,
    root: &Path,
) -> Vec<String> {
    semantic_issues_with_optional_context(value, Some(lock), Some(root))
}

fn semantic_issues_with_optional_context(
    value: &Value,
    lock: Option<&Value>,
    root: Option<&Path>,
) -> Vec<String> {
    domains::semantic_issues(value, lock, root)
}
