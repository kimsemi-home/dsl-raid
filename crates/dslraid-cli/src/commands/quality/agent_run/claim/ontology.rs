use super::id;
use crate::commands::quality::agent_run::fields::{field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    let Some(ssot) = text(value, &["ssot", "ontology_version"]) else {
        return;
    };
    let Some(version) = field_text(claim, "interpreted_under") else {
        return;
    };
    if version != ssot {
        issues.push(format!(
            "claim {} interpreted_under {version} differs from ssot {ssot}",
            id(claim)
        ));
    }
}
