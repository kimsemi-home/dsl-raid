use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, capacity: &Value, issues: &mut Vec<String>) {
    let refs = refs(capacity);
    if refs.is_empty() {
        issues.push("review capacity requires evidence".to_string());
    }
    let evidence = ids(value);
    for reference in refs {
        if !evidence.contains(reference) {
            issues.push(format!(
                "review capacity references unknown evidence {reference}"
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
