use super::super::fields::{field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let refs = authority_refs(value);
    if refs.is_empty() {
        issues.push("approved authority gate requires evidence".to_string());
    }
    let evidence = evidence_ids(value);
    for reference in refs {
        if !evidence.contains(reference) {
            issues.push(format!(
                "authority gate references unknown evidence {reference}"
            ));
        }
    }
}

fn authority_refs(value: &Value) -> Vec<&str> {
    value
        .get("authority_gate")
        .and_then(|gate| gate.get("evidence"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}

fn evidence_ids(value: &Value) -> BTreeSet<String> {
    items(value, "evidence")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}
