use crate::commands::quality::agent_run::fields::{field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, translation: &Value, issues: &mut Vec<String>) {
    let id = field_text(translation, "id").unwrap_or("<unknown>");
    let Some(actual) = field_text(translation, "interpreted_under") else {
        issues.push(format!("translation {id} requires interpreted_under"));
        return;
    };
    let Some(expected) = text(value, &["ssot", "ontology_version"]) else {
        return;
    };
    if actual != expected {
        issues.push(format!(
            "translation {id} interpreted_under {actual} differs from ssot {expected}"
        ));
    }
}
