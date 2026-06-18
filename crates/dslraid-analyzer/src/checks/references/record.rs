use serde_json::{json, Value};

use crate::builder::AssertionSpec;

pub(super) fn assertion(missing: Vec<Value>) -> AssertionSpec {
    let passed = missing.is_empty();
    AssertionSpec {
        proposition: "V002",
        assertion: "assertion:ir.reference_targets_exist",
        code: "IRR002",
        layer: "ir_structure",
        predicate: "reference_targets_exist",
        severity: "error",
        status: if passed { "passed" } else { "failed" },
        subjects: subjects(&missing),
        evidence: json!({ "missing": missing }),
        message: Some(super::message::text(passed).to_string()),
        suggestion: Some(super::message::suggestion().to_string()),
    }
}

fn subjects(missing: &[Value]) -> Vec<String> {
    missing
        .iter()
        .filter_map(|item| item.get("source")?.as_str())
        .map(str::to_string)
        .collect()
}
