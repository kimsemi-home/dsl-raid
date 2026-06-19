use std::path::{Path, PathBuf};

pub(super) fn path(stem: &str, extension: &str) -> PathBuf {
    std::env::temp_dir().join(format!("{stem}-{}.{}", std::process::id(), extension))
}

pub(super) fn remove(path: &Path) {
    std::fs::remove_file(path).ok();
}
