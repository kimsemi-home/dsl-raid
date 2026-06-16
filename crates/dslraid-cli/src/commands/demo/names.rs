use anyhow::{anyhow, Result};
use std::path::Path;

pub(super) fn source_map(input: &Path) -> Result<String> {
    Ok(format!("{}.sourcemap.json", design_stem(input)?))
}

pub(super) fn coverage(trace: &Path) -> Result<String> {
    Ok(format!("{}.coverage.json", trace_stem(trace)?))
}

fn design_stem(input: &Path) -> Result<String> {
    let name = file_name(input)?;
    Ok(name
        .strip_suffix(".raid.json")
        .or_else(|| name.strip_suffix(".dslraid.json"))
        .map(str::to_string)
        .unwrap_or_else(|| file_stem(input)))
}

fn trace_stem(trace: &Path) -> Result<String> {
    let name = file_name(trace)?;
    Ok(name
        .strip_suffix(".trace.json")
        .map(str::to_string)
        .unwrap_or_else(|| file_stem(trace)))
}

fn file_name(path: &Path) -> Result<&str> {
    path.file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| anyhow!("path has no UTF-8 file name: {}", path.display()))
}

fn file_stem(path: &Path) -> String {
    path.file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("dslraid")
        .to_string()
}
