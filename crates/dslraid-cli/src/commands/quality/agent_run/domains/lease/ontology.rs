use crate::commands::quality::agent_run::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let Some(lease_version) = text(value, &["lease", "ontology_version"]) else {
        issues.push("approved run requires lease ontology_version".to_string());
        return;
    };
    if text(value, &["ssot", "ontology_version"]).is_some_and(|ssot| ssot != lease_version) {
        issues.push("lease ontology_version differs from ssot ontology".to_string());
    }
}
