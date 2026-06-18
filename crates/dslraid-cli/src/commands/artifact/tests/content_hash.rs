use super::fixture::{runscope_fixture, runscope_lock, temp_lock_path};
use serde_json::Value;
use std::fs;

#[test]
fn artifact_verify_detects_stale_content_hash() {
    let mut lock: Value = serde_json::from_slice(&fs::read(runscope_lock()).unwrap()).unwrap();
    let stale_hash = "sha256:0000000000000000000000000000000000000000000000000000000000000001";
    lock["artifacts"][1]["content_hash"] = Value::String(stale_hash.to_string());
    let temp = temp_lock_path("stale-content-hash-test");
    fs::write(&temp, serde_json::to_vec_pretty(&lock).unwrap()).unwrap();

    let report = super::super::verify::build_report(&runscope_fixture(), Some(&temp)).unwrap();
    fs::remove_file(&temp).ok();

    assert_eq!(report.get("status").and_then(Value::as_str), Some("failed"));
    assert!(issues(&report).iter().any(|issue| {
        issue.get("code").and_then(Value::as_str) == Some("ART042")
            && issue.get("subject").and_then(Value::as_str) == Some("artifact:runtime_fsm.rs")
    }));
}

fn issues(report: &Value) -> &[Value] {
    report
        .get("issues")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .expect("report issues are present")
}
