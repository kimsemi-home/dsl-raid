use super::super::fields::{field_text, items};
use serde_json::Value;

pub(super) fn has_authority_subject(value: &Value, subject: &str) -> bool {
    authority_refs(value).iter().any(|reference| {
        items(value, "evidence").any(|evidence| {
            field_text(evidence, "id") == Some(reference.as_str())
                && field_text(evidence, "subject") == Some(subject)
        })
    })
}

fn authority_refs(value: &Value) -> Vec<String> {
    value
        .pointer("/authority_gate/evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}
