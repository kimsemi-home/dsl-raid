use super::fixture::{runscope_fixture, temp_lock_path};
use crate::commands::artifact::update_lock;
use serde_json::Value;
use std::fs;

#[test]
fn artifact_lock_update_writes_current_content_hashes() {
    let temp = temp_lock_path("lock-update-test");
    update_lock(&runscope_fixture(), Some(&temp)).unwrap();

    let lock: Value = serde_json::from_slice(&fs::read(&temp).unwrap()).unwrap();
    fs::remove_file(&temp).ok();

    let artifacts = lock
        .get("artifacts")
        .and_then(Value::as_array)
        .expect("artifact records are present");
    assert!(artifacts.iter().any(|record| {
        record.get("artifact").and_then(Value::as_str) == Some("artifact:runtime_fsm.rs")
            && record.get("content_hash").and_then(Value::as_str)
                == Some("sha256:0333c48f3cb811b37f6ce8812a7991d10980cc93f2552ac86fd8ec7b8bb0e556")
    }));
}
