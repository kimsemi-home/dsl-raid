use serde_json::Value;

pub(super) fn print_issues(report: &Value) {
    let Some(issues) = report.get("issues").and_then(Value::as_array) else {
        return;
    };
    for issue in issues {
        println!(
            "{} {} {}: {}",
            issue_text(issue, "severity", "error"),
            issue_text(issue, "code", "ART000"),
            issue_text(issue, "subject", "<unknown>"),
            issue_text(issue, "message", "")
        );
    }
}

fn issue_text<'a>(issue: &'a Value, key: &str, fallback: &'a str) -> &'a str {
    issue.get(key).and_then(Value::as_str).unwrap_or(fallback)
}
