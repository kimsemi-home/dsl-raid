use anyhow::Result;
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

pub(super) struct Preserved {
    pub(super) resolved_refs: Vec<Value>,
    pub(super) artifacts: BTreeMap<String, Value>,
    pub(super) golden: Vec<Value>,
    pub(super) compat: Vec<Value>,
}

pub(super) fn load(lock_path: &Path) -> Result<Preserved> {
    if !lock_path.exists() {
        return Ok(empty());
    }
    let value = super::super::lock::load_lock(lock_path)?;
    Ok(Preserved {
        resolved_refs: array(&value, "resolved_refs"),
        artifacts: super::super::lock::artifact_map(&value),
        golden: array(&value, "golden"),
        compat: array(&value, "compat"),
    })
}

fn empty() -> Preserved {
    Preserved {
        resolved_refs: Vec::new(),
        artifacts: BTreeMap::new(),
        golden: Vec::new(),
        compat: Vec::new(),
    }
}

fn array(value: &Value, field: &str) -> Vec<Value> {
    value
        .get(field)
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}
