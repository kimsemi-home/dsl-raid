use super::output_issue::print_issues;
use super::output_summary::print_artifact_summary;
use serde_json::Value;

pub(super) fn print_text(report: &Value) {
    println!(
        "artifact verification {}",
        report
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
    );
    println!(
        "input: {}",
        report
            .get("input")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>")
    );
    println!(
        "lock: {}",
        report
            .get("lock")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>")
    );
    print_artifact_summary(report);
    print_issues(report);
}
