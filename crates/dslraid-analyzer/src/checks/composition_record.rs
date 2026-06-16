use dslraid_core::CoreIr;
use serde_json::{json, Value};

use crate::builder::{AssertionSpec, ReportBuilder};

pub(crate) fn inputs(ir: &CoreIr, builder: &mut ReportBuilder, bad_inputs: Vec<Value>) {
    builder.record(AssertionSpec {
        proposition: "V021",
        assertion: "assertion:composition.inputs_are_fsms",
        code: "CMP021",
        layer: "composition",
        predicate: "composition_inputs_are_fsms",
        severity: "error",
        status: status(ir, &bad_inputs),
        subjects: subjects(&bad_inputs),
        evidence: json!({ "invalid_inputs": bad_inputs }),
        message: Some("Composition inputs must resolve to FSM objects.".to_string()),
        suggestion: Some("Reference fsm:* IDs in composition.inputs.".to_string()),
    });
}

pub(crate) fn policy(ir: &CoreIr, builder: &mut ReportBuilder, missing_policy: Vec<Value>) {
    builder.record(AssertionSpec {
        proposition: "V027",
        assertion: "assertion:composition.policy_explicit",
        code: "CMP027",
        layer: "composition",
        predicate: "composition_policy_explicit",
        severity: "error",
        status: status(ir, &missing_policy),
        subjects: subjects(&missing_policy),
        evidence: json!({ "missing_conflict_policy": missing_policy }),
        message: Some("Composition conflict policy must be explicit.".to_string()),
        suggestion: Some(
            "Add conflict_policy with deterministic nondeterminism handling.".to_string(),
        ),
    });
}

fn status(ir: &CoreIr, failures: &[Value]) -> &'static str {
    if ir.compositions.is_empty() {
        "not_applicable"
    } else if failures.is_empty() {
        "passed"
    } else {
        "failed"
    }
}

fn subjects(items: &[Value]) -> Vec<String> {
    items
        .iter()
        .filter_map(|item| item.get("composition").and_then(Value::as_str))
        .map(str::to_string)
        .collect()
}
