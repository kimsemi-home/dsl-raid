use super::id;
use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(
    agreement: &Value,
    producer: Option<&str>,
    reviewers: &BTreeSet<String>,
    issues: &mut Vec<String>,
) {
    let participants = participants(agreement);
    if participants.len() < 2 {
        issues.push(format!(
            "agreement {} requires at least two participants",
            id(agreement)
        ));
    }
    if producer.is_some_and(|id| !participants.iter().any(|item| item == id)) {
        issues.push(format!(
            "agreement {} requires producer participant",
            id(agreement)
        ));
    }
    if !participants.iter().any(|item| reviewers.contains(item)) {
        issues.push(format!(
            "agreement {} requires independent reviewer participant",
            id(agreement)
        ));
    }
}

pub(super) fn reviewer_ids(value: &Value) -> BTreeSet<String> {
    items(value, "reviewers")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

fn participants(value: &Value) -> Vec<String> {
    value
        .get("participants")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}
