use crate::commands::quality::agent_run::fields::text;
use serde_json::Value;

pub(super) fn is_high_risk(value: &Value) -> bool {
    matches!(
        text(value, &["authority_gate", "scope"]),
        Some("security" | "ontology" | "incident" | "authority")
    )
}

pub(super) fn is_automation_profile(value: &Value) -> bool {
    matches!(
        text(value, &["authority_gate", "profile"]),
        Some("automatic" | "sidecar")
    )
}
