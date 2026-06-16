use anyhow::{bail, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub(crate) fn check(path: &Path) -> Result<()> {
    ensure_exists(path)?;
    let mut checked = 0usize;
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            let _: Value = serde_json::from_slice(&fs::read(path)?)?;
            checked += 1;
        }
    }
    println!("golden ok: {checked} JSON fixture files checked");
    Ok(())
}

pub(crate) fn update(path: &Path) -> Result<()> {
    ensure_exists(path)?;
    println!(
        "golden update currently has no generated fixtures to refresh at {}",
        path.display()
    );
    Ok(())
}

fn ensure_exists(path: &Path) -> Result<()> {
    if path.exists() {
        Ok(())
    } else {
        bail!("golden path does not exist: {}", path.display())
    }
}
