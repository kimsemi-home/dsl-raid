use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    push_refs(
        item,
        "selected_reviewers",
        &ids(value, "reviewers"),
        "reviewer",
        issues,
    );
    push_refs(
        item,
        "input_evidence",
        &ids(value, "evidence"),
        "input evidence",
        issues,
    );
    push_refs(
        item,
        "output_artifacts",
        &ids(value, "artifacts"),
        "output artifact",
        issues,
    );
}

fn push_refs(
    item: &Value,
    key: &str,
    known: &BTreeSet<String>,
    label: &str,
    issues: &mut Vec<String>,
) {
    if known.is_empty() {
        return;
    }
    let refs = refs(item, key);
    if refs.is_empty() {
        issues.push(format!("orchestration receipt requires {key}"));
    }
    for reference in refs {
        if !known.contains(reference) {
            issues.push(format!(
                "orchestration references unknown {label} {reference}"
            ));
        }
    }
}

fn ids(value: &Value, key: &str) -> BTreeSet<String> {
    items(value, key)
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

fn refs<'a>(value: &'a Value, key: &str) -> Vec<&'a str> {
    value
        .get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}
