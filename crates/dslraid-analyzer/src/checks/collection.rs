mod message;
mod spec;
mod status;
mod subjects;

use serde_json::json;

use crate::builder::{AssertionSpec, ReportBuilder};

pub(crate) use spec::CollectionCheck;

pub(crate) fn record_collection_check(builder: &mut ReportBuilder, check: CollectionCheck<'_>) {
    builder.record(AssertionSpec {
        proposition: check.proposition,
        assertion: check.assertion,
        code: check.code,
        layer: check.layer,
        predicate: check.predicate,
        severity: check.severity,
        status: status::from_failures(check.failures, check.severity),
        subjects: subjects::from_failures(check.failures),
        evidence: json!({ "failures": check.failures }),
        message: Some(message::from_check(&check).to_string()),
        suggestion: Some(check.suggestion.to_string()),
    });
}
