use serde_json::Value;

pub(super) fn event_failed(event: &Value, kind: &str) -> bool {
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
