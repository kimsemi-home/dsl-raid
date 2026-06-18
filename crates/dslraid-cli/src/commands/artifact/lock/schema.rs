use std::path::{Path, PathBuf};

pub(super) fn schema_path() -> PathBuf {
    repo_relative_path("schemas/dslraid-lock.schema.json")
}

fn repo_relative_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.exists() {
        return direct;
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}
