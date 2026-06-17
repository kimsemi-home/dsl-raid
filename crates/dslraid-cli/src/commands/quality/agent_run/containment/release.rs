use super::{evidence, id};
use crate::commands::quality::agent_run::fields::{field_is, field_text};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(item: &Value, evidence_ids: &BTreeSet<String>, issues: &mut Vec<String>) {
    if !field_is(item, "status", "released") {
        return;
    }
    push_releaser_issue(item, issues);
    if release_conditions(item).next().is_none() {
        issues.push(format!(
            "released containment {} requires release conditions",
            id(item)
        ));
    }
    for condition in release_conditions(item) {
        push_condition_issues(item, condition, evidence_ids, issues);
    }
}

fn push_releaser_issue(item: &Value, issues: &mut Vec<String>) {
    let releaser = field_text(item, "released_by").unwrap_or("");
    if field_is(item, "kind", "quarantine") && !releaser.starts_with("steward:") {
        issues.push(format!(
            "released quarantine {} requires steward release",
            id(item)
        ));
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

fn release_conditions(value: &Value) -> impl Iterator<Item = &Value> {
    value
        .get("release_conditions")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
}
