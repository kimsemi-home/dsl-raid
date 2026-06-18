use crate::commands::quality::agent_run::fields::{field_text, text};
use serde_json::Value;

pub(super) fn matches(value: &Value, update: &Value) -> bool {
    let Some(owner) = field_text(update, "owner") else {
        return false;
    };
    text(value, &["authority_gate", "approved_by"]) == Some(owner)
}
