use crate::commands::quality::agent_run::fields::{field_text, text};
use serde_json::Value;

pub(super) fn matches(value: &Value, update: &Value) -> bool {
    text(value, &["ssot", "ontology_version"])
        .is_some_and(|version| field_text(update, "ontology_version") == Some(version))
}
