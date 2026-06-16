use std::collections::BTreeSet;

use dslraid_core::CoreIr;
use serde_json::json;

use crate::builder::{AssertionSpec, ReportBuilder};
use crate::subjects;

pub(crate) fn check(ir: &CoreIr, builder: &mut ReportBuilder) {
    let duplicates = duplicate_subjects(ir);
    builder.record(AssertionSpec {
        proposition: "V001",
        assertion: "assertion:ir.subject_ids_unique",
        code: "IRR001",
        layer: "ir_structure",
        predicate: "subject_ids_unique",
        severity: "error",
        status: if duplicates.is_empty() {
            "passed"
        } else {
            "failed"
        },
        subjects: duplicates.clone(),
        evidence: json!({ "duplicates": duplicates }),
        message: Some(message(duplicates.is_empty()).to_string()),
        suggestion: Some("Rename the duplicate object or assign a stable unique ID.".to_string()),
    });
}

fn duplicate_subjects(ir: &CoreIr) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut duplicates = Vec::new();
    for subject in subjects::all_declared(ir) {
        if !seen.insert(subject.clone()) {
            duplicates.push(subject);
        }
    }
    duplicates
}

fn message(passed: bool) -> &'static str {
    if passed {
        "All semantic subject IDs are unique."
    } else {
        "Duplicate semantic subject IDs were found."
    }
}
