mod evidence;
mod queue;
mod scope;

use super::super::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let high_risk = scope::is_high_risk(value);
    let Some(capacity) = value.get("review_capacity") else {
        if high_risk {
            issues.push("high-risk authority requires review capacity receipt".to_string());
        }
        return;
    };
    evidence::push_issues(value, capacity, issues);
    queue::push_issues(capacity, issues);
    push_freeze_issue(value, high_risk, capacity, issues);
}

fn push_freeze_issue(value: &Value, high_risk: bool, capacity: &Value, issues: &mut Vec<String>) {
    if !high_risk || !scope::is_automation_profile(value) {
        return;
    }
    if let Some(reason) = freeze_reason(capacity) {
        issues.push(format!(
            "review capacity {reason} freezes high-risk {} authority",
            text(value, &["authority_gate", "profile"]).unwrap_or("unknown")
        ));
    }
}

fn freeze_reason(capacity: &Value) -> Option<&str> {
    match text(capacity, &["status"]) {
        Some("overloaded") => Some("overloaded"),
        Some("frozen") => Some("frozen"),
        _ if queue::overflows(capacity) => Some("queue overflow"),
        _ => None,
    }
}
