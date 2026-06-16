use dslraid_core::SchemaIssue;
use serde_json::Value;

pub(super) fn print_schema_issues(schema_issues: &[SchemaIssue]) {
    for issue in schema_issues {
        println!("schema error at {}: {}", issue.instance_path, issue.message);
    }
}

pub(super) fn print_coverage_check_text(report: &Value) {
    if report.get("status").and_then(Value::as_str) == Some("passed") {
        println!("coverage check passed");
    } else {
        println!("coverage check failed");
        if let Some(issues) = report.get("issues").and_then(Value::as_array) {
            for issue in issues {
                print_issue(issue);
            }
        }
    }
}

fn print_issue(issue: &Value) {
    println!(
        "{} {}: {}",
        issue
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or("COV000"),
        issue
            .get("subject")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>"),
        issue.get("message").and_then(Value::as_str).unwrap_or("")
    );
}
