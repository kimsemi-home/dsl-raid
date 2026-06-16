use std::path::{Path, PathBuf};

pub(super) fn runscope_fixture() -> PathBuf {
    repo_path("examples/runscope/runscope.raid.json")
}

pub(super) fn runscope_lock() -> PathBuf {
    repo_path("examples/runscope/runscope.lock.json")
}

pub(super) fn temp_lock_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "dslraid-{name}-{}-{}.json",
        std::process::id(),
        timestamp()
    ))
}

fn repo_path(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

fn timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
