use serde_json::Value;

pub(super) fn from_failures(failures: &[Value]) -> Vec<String> {
    failures
        .iter()
        .filter_map(subject)
        .map(str::to_string)
        .collect()
}

fn subject(item: &Value) -> Option<&str> {
    item.get("transition")
        .or_else(|| item.get("projection"))
        .or_else(|| item.get("derivation"))
        .or_else(|| item.get("artifact"))
        .and_then(Value::as_str)
}
