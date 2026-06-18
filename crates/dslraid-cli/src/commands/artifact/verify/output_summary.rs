use serde_json::Value;

pub(super) fn print_artifact_summary(report: &Value) {
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
