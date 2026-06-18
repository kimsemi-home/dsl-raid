mod freshness;
mod ontology;
mod snapshot;

use super::super::fields::{field_is, field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let producer = text(value, &["producer", "id"]);
    let mut has_high_snapshot = false;
    for evidence in items(value, "evidence") {
        let snapshots = snapshot::collect(evidence);
        has_high_snapshot |= snapshot::has_high(&snapshots);
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
    if field_is(evidence, "quality", "high") && !snapshot::has_high(snapshots) {
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
    let Some(producer) = producer else {
        return;
    };
    if snapshot::has_assessor(snapshots, producer) {
        issues.push(format!(
            "evidence {} quality snapshot must be independent",
            evidence_id(evidence)
        ));
    }
}

fn evidence_id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
