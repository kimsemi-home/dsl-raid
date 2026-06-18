use crate::validate_json_file;
use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub(crate) fn load_lock(path: &Path) -> Result<Value> {
    validate_json_file(&super::schema::schema_path(), path)?;
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
