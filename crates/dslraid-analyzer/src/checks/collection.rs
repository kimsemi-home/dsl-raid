use serde_json::{json, Value};

use crate::builder::{AssertionSpec, ReportBuilder};

pub(crate) struct CollectionCheck<'a> {
    pub(crate) proposition: &'static str,
    pub(crate) assertion: &'static str,
    pub(crate) code: &'static str,
    pub(crate) layer: &'static str,
    pub(crate) predicate: &'static str,
    pub(crate) severity: &'static str,
    pub(crate) failures: &'a [Value],
    pub(crate) pass_message: &'static str,
    pub(crate) fail_message: &'static str,
    pub(crate) suggestion: &'static str,
}

pub(crate) fn record_collection_check(builder: &mut ReportBuilder, check: CollectionCheck<'_>) {
    builder.record(AssertionSpec {
        proposition: check.proposition,
        assertion: check.assertion,
        code: check.code,
        layer: check.layer,
        predicate: check.predicate,
        severity: check.severity,
        status: status(check.failures, check.severity),
        subjects: subjects(check.failures),
        evidence: json!({ "failures": check.failures }),
        message: Some(message(&check).to_string()),
        suggestion: Some(check.suggestion.to_string()),
    });
}

fn status(failures: &[Value], severity: &str) -> &'static str {
    if failures.is_empty() {
        "passed"
    } else if severity == "warning" {
        "warning"
    } else {
        "failed"
    }
}

fn message(check: &CollectionCheck<'_>) -> &'static str {
    if check.failures.is_empty() {
        check.pass_message
    } else {
        check.fail_message
    }
}

fn subjects(failures: &[Value]) -> Vec<String> {
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
