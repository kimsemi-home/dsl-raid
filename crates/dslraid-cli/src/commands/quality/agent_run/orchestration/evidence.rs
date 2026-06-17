use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    let refs = refs(item);
    if refs.is_empty() {
        issues.push("orchestration receipt requires evidence".to_string());
    }
    let known = ids(value);
    if known.is_empty() {
        return;
    }
    for reference in refs {
        if !known.contains(reference) {
            issues.push(format!(
                "orchestration references unknown evidence {reference}"
            ));
        }
    }
}

fn ids(value: &Value) -> BTreeSet<String> {
    items(value, "evidence")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

fn refs(value: &Value) -> Vec<&str> {
    value
        .get("evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}
