use super::design::coverage_design_issues;
use super::overlay::coverage_overlay_value;
use super::test_support::{failure_trace, find_subject, runscope_fixture, runscope_trace};
use dslraid_core::load_core_ir;
use serde_json::Value;
use std::fs;

#[test]
fn coverage_overlay_marks_fixture_transition_covered() {
    let ir = load_core_ir(runscope_fixture()).unwrap();
    let trace_path = runscope_trace();
    let trace_value: Value = serde_json::from_slice(&fs::read(&trace_path).unwrap()).unwrap();

    let coverage =
        coverage_overlay_value(&ir, &runscope_fixture(), &trace_path, &trace_value).unwrap();

    let subjects = coverage
        .get("subjects")
        .and_then(Value::as_array)
        .expect("coverage subjects are present");
    assert!(subjects.iter().any(|subject| {
        subject.get("subject").and_then(Value::as_str)
            == Some("transition:runtime.running_to_completed")
            && subject.get("status").and_then(Value::as_str) == Some("covered")
    }));
}

#[test]
fn coverage_design_issues_reports_missing_subject() {
    let ir = load_core_ir(runscope_fixture()).unwrap();
    let coverage = serde_json::json!({
        "subjects": [{
            "subject": "transition:runtime.idle_to_starting"
        }]
    });

    let issues = coverage_design_issues(&ir, &coverage).unwrap();

    assert!(issues
        .iter()
        .any(|issue| issue.get("code").and_then(Value::as_str) == Some("COV002")));
}

#[test]
fn coverage_overlay_reports_transition_failure_rate() {
    let ir = load_core_ir(runscope_fixture()).unwrap();
    let trace_path = runscope_trace();
    let trace_value = failure_trace();

    let coverage =
        coverage_overlay_value(&ir, &runscope_fixture(), &trace_path, &trace_value).unwrap();
    let subject = find_subject(&coverage, "transition:runtime.starting_to_failed");

    assert_eq!(subject.get("status").and_then(Value::as_str), Some("flaky"));
    assert_eq!(
        subject.get("failure_rate").and_then(Value::as_f64),
        Some(0.5)
    );
}
