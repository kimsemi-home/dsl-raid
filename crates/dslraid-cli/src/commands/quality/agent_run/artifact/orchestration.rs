use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let artifact_ids = ids(value);
    let output_ids = output_ids(value);
    if output_ids.is_empty() || !output_ids.is_subset(&artifact_ids) {
        return;
    }
    for artifact_id in artifact_ids.difference(&output_ids) {
        issues.push(format!(
            "artifact {artifact_id} must be listed in orchestration output_artifacts"
        ));
    }
}

fn ids(value: &Value) -> BTreeSet<String> {
    items(value, "artifacts")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}

fn output_ids(value: &Value) -> BTreeSet<String> {
    value
        .pointer("/orchestration/output_artifacts")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}
