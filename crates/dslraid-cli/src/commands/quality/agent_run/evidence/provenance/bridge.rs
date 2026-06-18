use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;

pub(super) fn covers(value: &Value, evidence_id: &str, expected: &str) -> bool {
    items(value, "translations").any(|translation| {
        references(translation, evidence_id)
            && interprets_current(translation, expected)
            && is_verified(translation)
            && carries_target_conformance(translation)
    })
}

fn references(translation: &Value, evidence_id: &str) -> bool {
    translation
        .get("evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|item| item.as_str() == Some(evidence_id))
}

fn interprets_current(translation: &Value, expected: &str) -> bool {
    field_text(translation, "interpreted_under") == Some(expected)
}

fn is_verified(translation: &Value) -> bool {
    matches!(
        field_text(translation, "status"),
        Some("verified" | "lossless")
    )
}

fn carries_target_conformance(translation: &Value) -> bool {
    matches!(
        field_text(translation, "conformance"),
        Some("target" | "dual")
    )
}
