use dslraid_core::sha256_bytes;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn resolve_artifact_path(input: &Path, artifact_path: &str) -> PathBuf {
    let path = Path::new(artifact_path);
    if path.is_absolute() {
        return path.to_path_buf();
    }
    input
        .ancestors()
        .map(|ancestor| ancestor.join(path))
        .find(|candidate| candidate.exists())
        .or_else(|| path.exists().then(|| path.to_path_buf()))
        .unwrap_or_else(|| path.to_path_buf())
}

pub(super) fn artifact_content_hash(input: &Path, artifact_path: &str) -> Option<String> {
    fs::read(resolve_artifact_path(input, artifact_path))
        .ok()
        .map(|bytes| sha256_bytes(&bytes))
}
