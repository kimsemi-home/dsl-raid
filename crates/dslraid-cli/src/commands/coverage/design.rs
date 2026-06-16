use super::missing::push_missing_design_subjects;
use anyhow::{anyhow, Result};
use dslraid_core::CoreIr;
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn coverage_design_issues(ir: &CoreIr, coverage_value: &Value) -> Result<Vec<Value>> {
    let known_subjects = ir.semantic_subjects();
    let mut issues = Vec::new();
    let covered_subjects = collect_covered_subjects(coverage_value, &known_subjects, &mut issues)?;
    push_missing_design_subjects(ir, &covered_subjects, &mut issues);
    Ok(issues)
}

fn collect_covered_subjects(
    coverage_value: &Value,
    known_subjects: &BTreeSet<String>,
    issues: &mut Vec<Value>,
) -> Result<BTreeSet<String>> {
    let mut covered_subjects = BTreeSet::new();
    for subject in coverage_value
        .get("subjects")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("coverage.subjects must be an array"))?
    {
        let Some(subject_id) = subject.get("subject").and_then(Value::as_str) else {
            continue;
        };
        if !known_subjects.contains(subject_id) {
            issues.push(serde_json::json!({
                "code": "COV001",
                "subject": subject_id,
                "message": "Coverage subject does not resolve to the design IR."
            }));
        }
        covered_subjects.insert(subject_id.to_string());
    }
    Ok(covered_subjects)
}
