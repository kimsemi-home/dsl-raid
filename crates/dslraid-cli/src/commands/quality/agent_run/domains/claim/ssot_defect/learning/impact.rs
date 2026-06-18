use crate::commands::quality::agent_run::fields::field_text;
use serde_json::Value;

pub(super) fn covers(update: &Value, claim: &Value) -> bool {
    let Some(subject) = field_text(claim, "subject") else {
        return false;
    };
    update
        .get("affected_subjects")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|item| item.as_str() == Some(subject))
}
