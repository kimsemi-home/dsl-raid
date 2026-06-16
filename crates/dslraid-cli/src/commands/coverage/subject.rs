use super::counter::CoverageCounter;
use serde_json::Value;

pub(super) fn coverage_subject_value(subject: String, counter: CoverageCounter) -> Option<Value> {
    if !is_coverage_kind(&counter.kind) {
        return None;
    }
    let failure_rate = failure_rate(&counter);
    let mut value = serde_json::json!({
        "subject": subject,
        "kind": counter.kind,
        "status": coverage_status(&counter),
        "count": counter.count,
        "failure_rate": failure_rate
    });
    if let Some(last_seen) = counter.last_seen {
        value
            .as_object_mut()
            .expect("coverage subject is an object")
            .insert("last_seen".to_string(), Value::String(last_seen));
    }
    Some(value)
}

fn is_coverage_kind(kind: &str) -> bool {
    matches!(
        kind,
        "state" | "transition" | "event" | "guard" | "action" | "artifact"
    )
}

fn coverage_status(counter: &CoverageCounter) -> String {
    if let Some(status) = &counter.status_override {
        return status.clone();
    }
    if counter.kind == "artifact" {
        return artifact_status(counter);
    }
    if counter.failures > 0 {
        "failed".to_string()
    } else if counter.count > 0 {
        "covered".to_string()
    } else {
        "uncovered".to_string()
    }
}

fn artifact_status(counter: &CoverageCounter) -> String {
    if counter.count > 0 {
        "deployed".to_string()
    } else {
        "not_deployed".to_string()
    }
}

fn failure_rate(counter: &CoverageCounter) -> f64 {
    if counter.count == 0 {
        0.0
    } else {
        counter.failures as f64 / counter.count as f64
    }
}
