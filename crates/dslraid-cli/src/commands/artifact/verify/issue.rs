use serde_json::Value;

pub(super) fn artifact_issue(
    code: &'static str,
    severity: &'static str,
    subject: &str,
    message: &str,
    actual: Option<&str>,
    expected: Option<&str>,
) -> Value {
    serde_json::json!({
        "code": code,
        "severity": severity,
        "subject": subject,
        "message": message,
        "actual": actual,
        "expected": expected
    })
}

pub(super) fn value_string(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}
