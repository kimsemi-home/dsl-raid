use super::super::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    push_scope_issue(value, issues);
    push_approver_issue(value, issues);
    push_steward_evidence_issue(value, issues);
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

fn push_steward_evidence_issue(value: &Value, issues: &mut Vec<String>) {
    if text(value, &["authority_gate", "scope"]) != Some("authority") {
        return;
    }
    if text(value, &["authority_gate", "profile"]) != Some("governance") {
        return;
    }
    let Some(approver) = text(value, &["authority_gate", "approved_by"]) else {
        return;
    };
    if approver.starts_with("steward:") && !has_approver_evidence(value, approver) {
        issues.push(format!(
            "governance steward approver {approver} requires authority evidence"
        ));
    }
}

fn has_approver_evidence(value: &Value, approver: &str) -> bool {
    authority_refs(value).iter().any(|reference| {
        items(value, "evidence").any(|evidence| {
            field_text(evidence, "id") == Some(reference.as_str())
                && field_text(evidence, "subject") == Some(approver)
        })
    })
}

fn authority_refs(value: &Value) -> Vec<String> {
    value
        .pointer("/authority_gate/evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}
