use super::events::apply_trace_event;
use super::seed::base_coverage_counters;
use super::subject::coverage_subject_value;
use super::value::value_string;
use anyhow::{anyhow, Result};
use dslraid_core::{sha256_json, CoreIr};
use serde_json::Value;
use std::path::Path;

pub(super) fn coverage_overlay_value(
    ir: &CoreIr,
    design_ir: &Path,
    trace: &Path,
    trace_value: &Value,
) -> Result<Value> {
    let mut counters = base_coverage_counters(ir);
    for event in trace_events(trace_value)? {
        apply_trace_event(&mut counters, event);
    }
    let mut subjects = counters
        .into_iter()
        .filter_map(|(subject, counter)| coverage_subject_value(subject, counter))
        .collect::<Vec<_>>();
    subjects.sort_by_key(|left| value_string(left, "subject"));
    Ok(serde_json::json!({
        "coverage_version": "0.1.0",
        "design_ir": {
            "path": design_ir.display().to_string(),
            "hash": sha256_json(ir)?
        },
        "traces": [{
            "path": trace.display().to_string(),
            "hash": sha256_json(trace_value)?
        }],
        "subjects": subjects,
        "metadata": {
            "generator": "dslraid-cli",
            "mode": "trace-derived"
        }
    }))
}

fn trace_events(trace_value: &Value) -> Result<&[Value]> {
    trace_value
        .get("events")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| anyhow!("trace.events must be an array"))
}
