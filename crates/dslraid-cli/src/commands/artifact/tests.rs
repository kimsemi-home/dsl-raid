use super::*;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn artifact_verify_passes_fixture_lock() {
    let report = report::build(&runscope_fixture(), Some(&runscope_lock())).unwrap();

    assert_eq!(report.get("status").and_then(Value::as_str), Some("passed"));
    assert_eq!(
        report.get("issues").and_then(Value::as_array).map(Vec::len),
        Some(0)
    );
}

#[test]
fn artifact_verify_detects_stale_input_hash() {
    let mut lock: Value = serde_json::from_slice(&fs::read(runscope_lock()).unwrap()).unwrap();
    let stale_hash = "sha256:0000000000000000000000000000000000000000000000000000000000000001";
    lock["core"]["ir_hash"] = Value::String(stale_hash.to_string());
    lock["artifacts"][0]["input_hash"] = Value::String(stale_hash.to_string());
    let temp = temp_lock_path();
    fs::write(&temp, serde_json::to_vec_pretty(&lock).unwrap()).unwrap();

    let report = report::build(&runscope_fixture(), Some(&temp)).unwrap();
    fs::remove_file(&temp).ok();

    assert_eq!(report.get("status").and_then(Value::as_str), Some("failed"));
    let issues = report
        .get("issues")
        .and_then(Value::as_array)
        .expect("report issues are present");
    assert!(issues
        .iter()
        .any(|issue| issue.get("code").and_then(Value::as_str) == Some("ART038")));
    assert!(issues
        .iter()
        .any(|issue| issue.get("code").and_then(Value::as_str) == Some("ART039")));
}

fn runscope_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/runscope/runscope.raid.json")
}

fn runscope_lock() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/runscope/runscope.lock.json")
}

fn temp_lock_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "dslraid-stale-lock-test-{}-{}.json",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}
