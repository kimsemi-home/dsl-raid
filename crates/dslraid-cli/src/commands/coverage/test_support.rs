use serde_json::Value;
use std::path::{Path, PathBuf};

pub(super) fn failure_trace() -> Value {
    serde_json::json!({
        "events": [
            {"kind": "transition_started", "subject": "transition:runtime.starting_to_failed"},
            {"kind": "transition_failed", "subject": "transition:runtime.starting_to_failed"}
        ]
    })
}

pub(super) fn find_subject<'a>(coverage: &'a Value, subject_id: &str) -> &'a Value {
    coverage
        .get("subjects")
        .and_then(Value::as_array)
        .unwrap()
        .iter()
        .find(|subject| subject.get("subject").and_then(Value::as_str) == Some(subject_id))
        .unwrap()
}

pub(super) fn runscope_fixture() -> PathBuf {
    root().join("examples/runscope/runscope.raid.json")
}

pub(super) fn runscope_trace() -> PathBuf {
    root().join("examples/runscope/run-001.trace.json")
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
