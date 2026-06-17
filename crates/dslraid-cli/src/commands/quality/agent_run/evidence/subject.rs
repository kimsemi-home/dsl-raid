use crate::commands::quality::agent_run::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let Some(run_id) = text(value, &["run", "id"]) else {
        return;
    };
    for evidence in items(value, "evidence") {
        if field_text(evidence, "subject") != Some(run_id) {
            issues.push(format!(
                "evidence {} subject differs from run {run_id}",
                evidence_id(evidence)
            ));
        }
    }
}

fn evidence_id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
