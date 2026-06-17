use crate::commands::quality::agent_run::fields::{field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    let Some(actual) = field_text(item, "ontology_version") else {
        issues.push("orchestration receipt requires ontology_version".to_string());
        return;
    };
    let Some(expected) = text(value, &["ssot", "ontology_version"]) else {
        return;
    };
    if actual != expected {
        issues.push(format!(
            "orchestration ontology_version {actual} differs from ssot {expected}"
        ));
    }
}
