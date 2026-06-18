use serde_json::Value;

pub(super) fn from_failures(failures: &[Value], severity: &str) -> &'static str {
    if failures.is_empty() {
        "passed"
    } else if severity == "warning" {
        "warning"
    } else {
        "failed"
    }
}
