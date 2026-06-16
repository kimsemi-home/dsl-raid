use super::consistency::push_transition_consistency_issues;
use anyhow::{anyhow, Result};
use dslraid_core::CoreIr;
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn trace_design_issues(ir: &CoreIr, trace_value: &Value) -> Result<Vec<Value>> {
    let known_subjects = ir.semantic_subjects();
    let mut issues = Vec::new();
    for event in trace_events(trace_value)? {
        push_unknown_reference_issues(event, &known_subjects, &mut issues);
        push_transition_consistency_issues(ir, event, &mut issues);
    }
    Ok(issues)
}

fn trace_events(trace_value: &Value) -> Result<&[Value]> {
    trace_value
        .get("events")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| anyhow!("trace.events must be an array"))
}

fn push_unknown_reference_issues(
    event: &Value,
    known_subjects: &BTreeSet<String>,
    issues: &mut Vec<Value>,
) {
    let event_id = event_id(event);
    for field in ["subject", "from", "to"] {
        if let Some(subject) = event.get(field).and_then(Value::as_str) {
            if !known_subjects.contains(subject) {
                issues.push(serde_json::json!({
                    "code": "RTE049",
                    "event": event_id,
                    "field": field,
                    "subject": subject,
                    "message": "runtime trace event does not map to a known design subject"
                }));
            }
        }
    }
}

pub(super) fn event_id(event: &Value) -> &str {
    event
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("<unknown>")
}
