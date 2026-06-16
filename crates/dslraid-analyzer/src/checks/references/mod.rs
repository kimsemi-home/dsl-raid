mod collect;

use dslraid_core::CoreIr;
use serde_json::{json, Value};

use crate::builder::{AssertionSpec, ReportBuilder};

pub(crate) fn check(ir: &CoreIr, builder: &mut ReportBuilder) {
    let missing = collect::missing_refs(ir);
    let subjects = missing
        .iter()
        .filter_map(|item| item.get("source").and_then(Value::as_str))
        .map(str::to_string)
        .collect();

    builder.record(AssertionSpec {
        proposition: "V002",
        assertion: "assertion:ir.reference_targets_exist",
        code: "IRR002",
        layer: "ir_structure",
        predicate: "reference_targets_exist",
        severity: "error",
        status: if missing.is_empty() {
            "passed"
        } else {
            "failed"
        },
        subjects,
        evidence: json!({ "missing": missing }),
        message: Some(message(missing.is_empty()).to_string()),
        suggestion: Some(
            "Add the missing subject or update the reference to a stable existing ID.".to_string(),
        ),
    });
}

fn message(passed: bool) -> &'static str {
    if passed {
        "All semantic references resolve."
    } else {
        "Some semantic references do not resolve."
    }
}
