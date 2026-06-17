use super::super::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    push_scope_issue(value, issues);
    push_approver_issue(value, issues);
}

fn push_scope_issue(value: &Value, issues: &mut Vec<String>) {
    if text(value, &["authority_gate", "scope"]) != Some("authority") {
        return;
    }
    if text(value, &["authority_gate", "profile"]) != Some("governance") {
        issues.push("authority scope authority requires governance profile".to_string());
    }
}

fn push_approver_issue(value: &Value, issues: &mut Vec<String>) {
    if text(value, &["authority_gate", "profile"]) != Some("governance") {
        return;
    }
    let approver = text(value, &["authority_gate", "approved_by"]).unwrap_or("");
    if !approver.starts_with("steward:") {
        issues.push("governance authority profile requires steward approver".to_string());
    }
}
