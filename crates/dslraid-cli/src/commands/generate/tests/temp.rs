use std::path::PathBuf;

pub(super) fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "dslraid-generate-{name}-{}-{}.json",
        std::process::id(),
        timestamp()
    ))
}

pub(super) fn temp_dir(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "dslraid-generate-{name}-{}-{}",
        std::process::id(),
        timestamp()
    ))
}

fn timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
