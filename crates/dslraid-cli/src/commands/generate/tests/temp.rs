use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);

pub(super) fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "dslraid-generate-{name}-{}-{}-{}.json",
        std::process::id(),
        timestamp(),
        next_id()
    ))
}

pub(super) fn temp_dir(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "dslraid-generate-{name}-{}-{}-{}",
        std::process::id(),
        timestamp(),
        next_id()
    ))
}

fn next_id() -> u64 {
    NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed)
}

fn timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
