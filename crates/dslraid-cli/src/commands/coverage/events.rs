use super::counter::{mark_coverage, CoverageCounters};
use serde_json::Value;

pub(super) fn apply_trace_event(counters: &mut CoverageCounters, event: &Value) {
    let kind = event
        .get("kind")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let timestamp = event
        .get("timestamp")
        .and_then(Value::as_str)
        .map(str::to_string);
    let failed = event_failed(event, kind);

    match kind {
        "event_received"
        | "state_entered"
        | "state_exited"
        | "transition_started"
        | "transition_completed"
        | "transition_failed"
        | "action_started"
        | "action_completed"
        | "diagnostic_emitted" => {
            mark_event_subject(counters, event, failed, timestamp.clone(), None);
            mark_transition_endpoint_subjects(counters, event, kind, timestamp);
        }
        "artifact_deployed" => {
            mark_event_subject(counters, event, failed, timestamp, Some("deployed"));
        }
        _ => {}
    }
}

fn mark_event_subject(
    counters: &mut CoverageCounters,
    event: &Value,
    failed: bool,
    timestamp: Option<String>,
    status_override: Option<&str>,
) {
    if let Some(subject) = event.get("subject").and_then(Value::as_str) {
        mark_coverage(counters, subject, failed, timestamp, status_override);
    }
}

fn mark_transition_endpoint_subjects(
    counters: &mut CoverageCounters,
    event: &Value,
    kind: &str,
    timestamp: Option<String>,
) {
    if !matches!(
        kind,
        "transition_started" | "transition_completed" | "transition_failed"
    ) {
        return;
    }
    for field in ["from", "to"] {
        if let Some(subject) = event.get(field).and_then(Value::as_str) {
            mark_coverage(counters, subject, false, timestamp.clone(), None);
        }
    }
}

fn event_failed(event: &Value, kind: &str) -> bool {
    event
        .get("status")
        .and_then(Value::as_str)
        .is_some_and(|status| {
            matches!(
                status,
                "failed" | "timeout" | "cancelled" | "policy_blocked" | "degraded"
            )
        })
        || kind == "transition_failed"
}
