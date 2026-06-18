use std::path::{Path, PathBuf};

pub(crate) fn inferred_lock_path(input: &Path) -> PathBuf {
    let Some(file_name) = input.file_name().and_then(|name| name.to_str()) else {
        return input.with_extension("lock.json");
    };
    let lock_name = lock_name(file_name);
    input
        .parent()
        .map(|parent| parent.join(&lock_name))
        .unwrap_or_else(|| PathBuf::from(lock_name))
}

fn lock_name(file_name: &str) -> String {
    if let Some(prefix) = file_name.strip_suffix(".raid.json") {
        format!("{prefix}.lock.json")
    } else if let Some(prefix) = file_name.strip_suffix(".dslraid.json") {
        format!("{prefix}.dslraid.lock.json")
    } else if let Some(prefix) = file_name.strip_suffix(".json") {
        format!("{prefix}.lock.json")
    } else {
        format!("{file_name}.lock.json")
    }
}
