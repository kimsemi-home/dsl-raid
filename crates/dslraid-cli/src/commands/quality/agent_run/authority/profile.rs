use super::super::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let profile = text(value, &["authority_gate", "profile"]);
    let scope = text(value, &["authority_gate", "scope"]);
    if profile.is_none() {
        issues.push("approved authority gate requires profile".to_string());
    }
    if scope.is_none() {
        issues.push("approved authority gate requires scope".to_string());
    }
    push_human_review_issue(value, issues);
    push_scope_issue(value, scope, issues);
    push_automatic_issue(profile, scope, issues);
}

fn push_human_review_issue(value: &Value, issues: &mut Vec<String>) {
    if value
        .pointer("/authority_gate/human_review_required")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return;
    }
    let approver = text(value, &["authority_gate", "approved_by"]).unwrap_or("");
    if !approver.starts_with("human:") && !approver.starts_with("steward:") {
        issues.push("human review authority gate requires human or steward approver".to_string());
    }
}

fn push_scope_issue(value: &Value, scope: Option<&str>, issues: &mut Vec<String>) {
    if !matches!(
        scope,
        Some("security" | "ontology" | "incident" | "authority")
    ) {
        return;
    }
    if value
        .pointer("/authority_gate/human_review_required")
        .and_then(Value::as_bool)
        != Some(true)
    {
        issues.push(format!(
            "authority scope {} requires human review",
            scope.unwrap()
        ));
    }
}

fn push_automatic_issue(profile: Option<&str>, scope: Option<&str>, issues: &mut Vec<String>) {
    if profile == Some("automatic") && scope != Some("routine") {
        issues.push("automatic authority profile can only approve routine scope".to_string());
    }
}
