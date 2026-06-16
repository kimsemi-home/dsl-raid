use super::design::event_id;
use super::transition::transition_endpoints;
use dslraid_core::CoreIr;
use serde_json::Value;

pub(super) fn push_transition_consistency_issues(
    ir: &CoreIr,
    event: &Value,
    issues: &mut Vec<Value>,
) {
    if !is_transition_event(event) {
        return;
    }
    let event_id = event_id(event);
    let Some(subject) = event.get("subject").and_then(Value::as_str) else {
        issues.push(serde_json::json!({
            "code": "RTE049",
            "event": event_id,
            "message": "transition trace event is missing subject"
        }));
        return;
    };
    let Some((from, to)) = transition_endpoints(ir, subject) else {
        return;
    };
    push_endpoint_contradiction(event, issues, event_id, subject, "from", &from);
    push_endpoint_contradiction(event, issues, event_id, subject, "to", &to);
}

fn push_endpoint_contradiction(
    event: &Value,
    issues: &mut Vec<Value>,
    event_id: &str,
    subject: &str,
    field: &str,
    expected: &str,
) {
    if event
        .get(field)
        .and_then(Value::as_str)
        .is_some_and(|value| value != expected)
    {
        issues.push(serde_json::json!({
            "code": "RTE050",
            "event": event_id,
            "subject": subject,
            "message": format!("trace {field}-state contradicts transition definition"),
            "expected": expected,
            "actual": event.get(field)
        }));
    }
}

fn is_transition_event(event: &Value) -> bool {
    matches!(
        event.get("kind").and_then(Value::as_str),
        Some("transition_started" | "transition_completed" | "transition_failed")
    )
}
