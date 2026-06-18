use super::id;
use crate::commands::quality::agent_run::fields::field_text;
use serde_json::Value;

pub(super) fn push_issues(agreement: &Value, run_id: Option<&str>, issues: &mut Vec<String>) {
    let Some(run_id) = run_id else {
        return;
    };
    if field_text(agreement, "subject") != Some(run_id) {
        issues.push(format!(
            "agreement {} subject differs from run {run_id}",
            id(agreement)
        ));
    }
}
