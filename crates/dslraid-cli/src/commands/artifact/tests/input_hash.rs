use super::fixture::{runscope_fixture, runscope_lock, temp_lock_path};
use crate::commands::artifact::report;
use serde_json::Value;
use std::fs;

#[test]
fn artifact_verify_detects_stale_input_hash() {
    let mut lock: Value = serde_json::from_slice(&fs::read(runscope_lock()).unwrap()).unwrap();
    let stale_hash = "sha256:0000000000000000000000000000000000000000000000000000000000000001";
    lock["core"]["ir_hash"] = Value::String(stale_hash.to_string());
    lock["artifacts"][0]["input_hash"] = Value::String(stale_hash.to_string());
    let temp = temp_lock_path("stale-input-hash-test");
    fs::write(&temp, serde_json::to_vec_pretty(&lock).unwrap()).unwrap();

    let report = report::build(&runscope_fixture(), Some(&temp)).unwrap();
    fs::remove_file(&temp).ok();

    assert_eq!(report.get("status").and_then(Value::as_str), Some("failed"));
    assert_issue(&report, "ART038");
    assert_issue(&report, "ART039");
}

fn assert_issue(report: &Value, code: &str) {
    let issues = report
        .get("issues")
        .and_then(Value::as_array)
        .expect("report issues are present");
    assert!(issues
        .iter()
        .any(|issue| issue.get("code").and_then(Value::as_str) == Some(code)));
}
