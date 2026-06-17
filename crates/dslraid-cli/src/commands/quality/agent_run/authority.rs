mod evidence;
mod governance;
mod policy;
mod profile;

use super::fields::text;
use serde_json::Value;

pub(super) fn push_verified_gate_issue(value: &Value, issues: &mut Vec<String>) {
    if text(value, &["run", "status"]) == Some("verified") && !is_approved(value) {
        issues.push("verified run requires approved authority gate".to_string());
    }
}

pub(super) fn is_approved(value: &Value) -> bool {
    text(value, &["authority_gate", "decision"]) == Some("approved")
}

pub(super) fn push_approved_issues(value: &Value, issues: &mut Vec<String>) {
    push_self_approval_issue(value, issues);
    evidence::push_issues(value, issues);
    governance::push_issues(value, issues);
    policy::push_issues(value, issues);
    profile::push_issues(value, issues);
}

fn push_self_approval_issue(value: &Value, issues: &mut Vec<String>) {
    let producer = text(value, &["producer", "id"]);
    if text(value, &["authority_gate", "approved_by"]) == producer {
        issues.push("producer cannot approve its own output".to_string());
    }
}
