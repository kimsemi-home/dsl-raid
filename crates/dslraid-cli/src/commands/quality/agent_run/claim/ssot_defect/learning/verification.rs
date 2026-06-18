use crate::commands::quality::agent_run::fields::field_text;
use serde_json::Value;

pub(super) fn matches(update: &Value, claim: &Value) -> bool {
    let Some(plan) = field_text(claim, "verification_plan") else {
        return false;
    };
    field_text(update, "verification_plan") == Some(plan)
}
