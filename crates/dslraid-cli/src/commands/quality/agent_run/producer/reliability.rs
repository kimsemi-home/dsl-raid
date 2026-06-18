use crate::commands::quality::agent_run::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if !matches!(text(value, &["producer", "trust_tier"]), Some("T3" | "T4")) {
        return;
    }
    let Some(producer) = text(value, &["producer", "id"]) else {
        return;
    };
    if !has_authority_evidence(value, producer) {
        issues.push(format!(
            "trusted producer {producer} requires reliability evidence"
        ));
    }
}

fn has_authority_evidence(value: &Value, producer: &str) -> bool {
    authority_refs(value).iter().any(|reference| {
        items(value, "evidence").any(|evidence| {
            field_text(evidence, "id") == Some(reference.as_str())
                && field_text(evidence, "subject") == Some(producer)
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
