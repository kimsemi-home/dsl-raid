use anyhow::Result;
use dslraid_core::CORE_SCHEMA_PATH;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub(super) fn check() -> Result<()> {
    for input in core_input_files(Path::new("tests/golden"))? {
        crate::schema_validate(Path::new(CORE_SCHEMA_PATH), &input)?;
    }
    Ok(())
}

fn core_input_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    if !root.exists() {
        return Ok(paths);
    }
    for entry in WalkDir::new(root) {
        let entry = entry?;
        if entry.file_type().is_file() && is_core_input(entry.path()) {
            paths.push(entry.path().to_path_buf());
        }
    }
    paths.sort();
    Ok(paths)
}

fn is_core_input(path: &Path) -> bool {
    path.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|name| name.ends_with(".input.json"))
}
