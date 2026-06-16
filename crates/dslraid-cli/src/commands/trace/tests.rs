use super::design::trace_design_issues;
use super::transition::transition_endpoints;
use dslraid_core::load_core_ir;
use serde_json::Value;
use std::path::{Path, PathBuf};

#[test]
fn transition_endpoints_resolves_fixture_transition() {
    let ir = load_core_ir(runscope_fixture()).unwrap();

    assert_eq!(
        transition_endpoints(&ir, "transition:runtime.idle_to_starting"),
        Some((
            "state:runtime.idle".to_string(),
            "state:runtime.starting".to_string()
        ))
    );
}

#[test]
fn trace_design_issues_detects_contradicting_to_state() {
    let ir = load_core_ir(runscope_fixture()).unwrap();
    let trace = serde_json::json!({
        "events": [{
            "id": "evt-test",
            "kind": "transition_completed",
            "subject": "transition:runtime.idle_to_starting",
            "from": "state:runtime.idle",
            "to": "state:runtime.running"
        }]
    });

    let issues = trace_design_issues(&ir, &trace).unwrap();

    assert!(issues
        .iter()
        .any(|issue| issue.get("code").and_then(Value::as_str) == Some("RTE050")));
}

fn runscope_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/runscope/runscope.raid.json")
}
