use std::path::{Path, PathBuf};

pub(super) fn source_roots(root: &Path) -> [PathBuf; 4] {
    [
        root.join("crates"),
        root.join("apps/viewer/src"),
        root.join("lisp"),
        root.join("scripts"),
    ]
}

pub(super) fn line_limit() -> usize {
    std::env::var("SOURCE_LINE_LIMIT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(75)
}
