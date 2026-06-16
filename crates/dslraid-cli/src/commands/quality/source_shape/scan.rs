use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub(super) struct Failure {
    pub(super) path: PathBuf,
    pub(super) lines: usize,
}

pub(super) fn failures(root: &Path, limit: usize) -> Result<Vec<Failure>> {
    let mut failures = Vec::new();
    for source_root in super::roots::source_roots(root) {
        if source_root.exists() {
            scan_root(&source_root, root, limit, &mut failures)?;
        }
    }
    Ok(failures)
}

fn scan_root(root: &Path, repo: &Path, limit: usize, failures: &mut Vec<Failure>) -> Result<()> {
    for entry in WalkDir::new(root).into_iter().filter_entry(included) {
        let entry = entry?;
        if entry.file_type().is_file() && is_source(entry.path()) {
            record(entry.path(), repo, limit, failures)?;
        }
    }
    Ok(())
}

fn record(path: &Path, repo: &Path, limit: usize, failures: &mut Vec<Failure>) -> Result<()> {
    let lines = super::count::lines(path)?;
    if lines > limit {
        failures.push(Failure {
            path: path.strip_prefix(repo).unwrap_or(path).to_path_buf(),
            lines,
        });
    }
    Ok(())
}

fn included(entry: &DirEntry) -> bool {
    !matches!(
        entry.file_name().to_str(),
        Some("node_modules" | "dist" | "target")
    )
}

fn is_source(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|value| value.to_str()),
        Some("rs" | "ts" | "tsx" | "js" | "jsx" | "lisp" | "sh")
    )
}
