use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

#[test]
fn package_writes_viewer_demo_assets() {
    let out = temp_dir();
    super::package::run(
        &repo_path("examples/runscope/runscope.raid.json"),
        &out,
        None,
    )
    .unwrap();

    assert!(out.join("runscope.raid.json").exists());
    assert!(out.join("runscope.raid.view.json").exists());
    assert!(out.join("runscope.raid.svg").exists());
    assert!(out.join("runscope.sourcemap.json").exists());
    fs::remove_dir_all(out).ok();
}

#[test]
fn package_trace_writes_coverage_overlay() {
    let out = temp_dir();
    super::package::run(
        &repo_path("examples/runscope/runscope.raid.json"),
        &out,
        Some(&repo_path("examples/runscope/run-001.trace.json")),
    )
    .unwrap();

    let coverage = read_json(&out.join("run-001.coverage.json"));
    assert_eq!(coverage["coverage_version"].as_str(), Some("0.1.0"));
    assert!(out.join("run-001.trace.json").exists());
    fs::remove_dir_all(out).ok();
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn repo_path(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

fn temp_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "dslraid-demo-{}-{}-{}",
        std::process::id(),
        timestamp(),
        TEMP_ID.fetch_add(1, Ordering::SeqCst)
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
