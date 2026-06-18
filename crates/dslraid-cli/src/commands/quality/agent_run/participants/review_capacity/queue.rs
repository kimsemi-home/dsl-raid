use serde_json::Value;

pub(super) fn push_issues(capacity: &Value, issues: &mut Vec<String>) {
    if overflows(capacity) {
        issues.push("review capacity queue depth exceeds max".to_string());
    }
}

pub(super) fn overflows(capacity: &Value) -> bool {
    let depth = capacity.get("queue_depth").and_then(Value::as_u64);
    let max = capacity.get("max_queue_depth").and_then(Value::as_u64);
    depth.zip(max).is_some_and(|(depth, max)| depth > max)
}
