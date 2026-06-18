use super::super::{evidence, id};
use crate::commands::quality::agent_run::fields::field_is;
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(item: &Value, evidence_ids: &BTreeSet<String>, issues: &mut Vec<String>) {
    let conditions = release_conditions(item);
    if conditions.is_empty() {
        issues.push(format!(
            "released containment {} requires release conditions",
            id(item)
        ));
    }
    for condition in conditions {
        push_condition_issues(item, condition, evidence_ids, issues);
    }
}

fn push_condition_issues(
    item: &Value,
    condition: &Value,
    evidence_ids: &BTreeSet<String>,
    issues: &mut Vec<String>,
) {
    if !field_is(condition, "status", "met") {
        issues.push(format!(
            "released containment {} has unmet release condition",
            id(item)
        ));
    }
    if evidence::refs(condition).is_empty() {
        issues.push(format!(
            "release condition {} requires evidence",
            id(condition)
        ));
    }
    let refs = evidence::refs(condition);
    evidence::push_unknown(
        "release condition",
        id(condition),
        refs,
        evidence_ids,
        issues,
    );
}

fn release_conditions(value: &Value) -> Vec<&Value> {
    value
        .get("release_conditions")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .collect()
}
