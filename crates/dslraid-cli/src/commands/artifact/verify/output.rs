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

fn print_artifact_summary(report: &Value) {
    let Some(artifacts) = report.get("artifacts").and_then(Value::as_array) else {
        return;
    };
    let fresh = count_status(artifacts, "fresh");
    let stale = count_status(artifacts, "stale");
    let missing = count_status(artifacts, "missing");
    let external = count_status(artifacts, "external");
    println!("artifacts: fresh={fresh} stale={stale} missing={missing} external={external}");
}

fn count_status(artifacts: &[Value], status: &str) -> usize {
    artifacts
        .iter()
        .filter(|artifact| artifact.get("status").and_then(Value::as_str) == Some(status))
        .count()
}

fn print_issues(report: &Value) {
    let Some(issues) = report.get("issues").and_then(Value::as_array) else {
        return;
    };
    for issue in issues {
        println!(
            "{} {} {}: {}",
            issue
                .get("severity")
                .and_then(Value::as_str)
                .unwrap_or("error"),
            issue
                .get("code")
                .and_then(Value::as_str)
                .unwrap_or("ART000"),
            issue
                .get("subject")
                .and_then(Value::as_str)
                .unwrap_or("<unknown>"),
            issue.get("message").and_then(Value::as_str).unwrap_or("")
        );
    }
}
