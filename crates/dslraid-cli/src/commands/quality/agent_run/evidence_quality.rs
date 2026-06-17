mod freshness;
mod ontology;

use super::fields::{field_is, field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let producer = text(value, &["producer", "id"]);
    let mut has_high_snapshot = false;
    for evidence in items(value, "evidence") {
        let snapshots = snapshots(evidence);
        has_high_snapshot |= snapshots.iter().any(is_high_snapshot);
        push_claim_issue(evidence, &snapshots, issues);
        push_assessor_issue(evidence, &snapshots, producer, issues);
        freshness::push_issues(evidence, &snapshots, issues);
        ontology::push_issues(value, evidence, &snapshots, issues);
    }
    if !has_high_snapshot {
        issues.push("approved run requires high quality evidence snapshot".to_string());
    }
}

fn push_claim_issue(evidence: &Value, snapshots: &[&Value], issues: &mut Vec<String>) {
    if field_is(evidence, "quality", "high") && !snapshots.iter().any(is_high_snapshot) {
        issues.push(format!(
            "evidence {} requires high quality snapshot",
            evidence_id(evidence)
        ));
    }
}

fn push_assessor_issue(
    evidence: &Value,
    snapshots: &[&Value],
    producer: Option<&str>,
    issues: &mut Vec<String>,
) {
    if producer.is_none() {
        return;
    }
    if snapshots
        .iter()
        .any(|item| field_text(item, "assessor") == producer)
    {
        issues.push(format!(
            "evidence {} quality snapshot must be independent",
            evidence_id(evidence)
        ));
    }
}

fn snapshots(value: &Value) -> Vec<&Value> {
    value
        .get("quality_snapshots")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .collect()
}

fn is_high_snapshot(value: &&Value) -> bool {
    field_is(value, "quality", "high")
}

fn evidence_id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
