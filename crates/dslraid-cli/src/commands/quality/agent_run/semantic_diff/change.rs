use super::id;
use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    if !field_is(item, "status", "changed") {
        return;
    }
    let refs = refs(item);
    if refs.is_empty() || refs.iter().any(|reference| !has_evidence(value, reference)) {
        return;
    }
    if !refs
        .iter()
        .any(|reference| has_kind(value, reference, "validation"))
    {
        issues.push(format!(
            "changed semantic diff {} requires validation evidence",
            id(item)
        ));
    }
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

fn has_evidence(value: &Value, reference: &str) -> bool {
    items(value, "evidence").any(|item| field_text(item, "id") == Some(reference))
}

fn has_kind(value: &Value, reference: &str, kind: &str) -> bool {
    items(value, "evidence")
        .any(|item| field_text(item, "id") == Some(reference) && field_is(item, "kind", kind))
}
