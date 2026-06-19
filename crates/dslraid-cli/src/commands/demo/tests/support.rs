use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

pub(super) fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

pub(super) fn repo_path(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

pub(super) fn temp_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(name());
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub(super) fn remove_dir(path: PathBuf) {
    std::fs::remove_dir_all(path).ok();
}

fn name() -> String {
    format!(
        "dslraid-demo-{}-{}-{}",
        std::process::id(),
        timestamp(),
        TEMP_ID.fetch_add(1, Ordering::SeqCst)
    )
}

fn timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
