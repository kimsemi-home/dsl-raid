use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub(super) fn check_json_syntax(path: impl AsRef<Path>) -> Result<()> {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            parse(path)?;
        }
    }
    Ok(())
}

fn parse(path: &Path) -> Result<()> {
    let _: Value = serde_json::from_slice(
        &fs::read(path).with_context(|| format!("read {}", path.display()))?,
    )
    .with_context(|| format!("parse {}", path.display()))?;
    Ok(())
}
