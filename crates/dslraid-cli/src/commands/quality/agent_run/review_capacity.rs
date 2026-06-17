mod evidence;
mod scope;

use super::fields::text;
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
    push_queue_issue(capacity, issues);
    push_freeze_issue(value, high_risk, capacity, issues);
}

fn push_queue_issue(capacity: &Value, issues: &mut Vec<String>) {
    let depth = capacity.get("queue_depth").and_then(Value::as_u64);
    let max = capacity.get("max_queue_depth").and_then(Value::as_u64);
    if depth.zip(max).is_some_and(|(depth, max)| depth > max) {
        issues.push("review capacity queue depth exceeds max".to_string());
    }
}

fn push_freeze_issue(value: &Value, high_risk: bool, capacity: &Value, issues: &mut Vec<String>) {
    if !high_risk || !scope::is_automation_profile(value) {
        return;
    }
    if matches!(text(capacity, &["status"]), Some("overloaded" | "frozen")) {
        issues.push(format!(
            "review capacity {} freezes high-risk {} authority",
            text(capacity, &["status"]).unwrap(),
            text(value, &["authority_gate", "profile"]).unwrap_or("unknown")
        ));
    }
}
