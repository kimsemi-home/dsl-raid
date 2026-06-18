use super::fixture::{runscope_fixture, runscope_lock};
use serde_json::Value;

#[test]
fn artifact_verify_passes_fixture_lock() {
    let report =
        super::super::verify::build_report(&runscope_fixture(), Some(&runscope_lock())).unwrap();

    assert_eq!(report.get("status").and_then(Value::as_str), Some("passed"));
    assert_eq!(
        report.get("issues").and_then(Value::as_array).map(Vec::len),
        Some(0)
    );
}
