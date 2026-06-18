use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, shadow: &Value, issues: &mut Vec<String>) {
    let refs = refs(shadow);
    if refs.is_empty() {
        issues.push("shadow orchestration requires evidence".to_string());
    }
    let known = evidence_ids(value);
    for reference in refs {
        if !known.contains(reference) {
            issues.push(format!(
                "shadow orchestration references unknown evidence {reference}"
            ));
        }
    }
}

fn refs(value: &Value) -> Vec<&str> {
    items(value, "evidence").filter_map(Value::as_str).collect()
}

fn evidence_ids(value: &Value) -> BTreeSet<String> {
    items(value, "evidence")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}
