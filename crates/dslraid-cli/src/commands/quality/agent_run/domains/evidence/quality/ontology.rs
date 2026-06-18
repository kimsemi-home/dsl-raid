use super::super::super::fields::{field_is, field_text, text};
use serde_json::Value;

pub(super) fn push_issues(
    value: &Value,
    evidence: &Value,
    snapshots: &[&Value],
    issues: &mut Vec<String>,
) {
    let Some(ssot) = text(value, &["ssot", "ontology_version"]) else {
        return;
    };
    for snapshot in snapshots
        .iter()
        .filter(|item| field_is(item, "quality", "high"))
    {
        push_snapshot_issue(evidence_id(evidence), snapshot, ssot, issues);
    }
}

fn push_snapshot_issue(id: &str, snapshot: &Value, ssot: &str, issues: &mut Vec<String>) {
    let Some(version) = field_text(snapshot, "ontology_version") else {
        issues.push(format!(
            "evidence {id} high quality snapshot requires ontology_version"
        ));
        return;
    };
    if version != ssot {
        issues.push(format!(
            "evidence {id} high quality snapshot ontology {version} differs from ssot {ssot}"
        ));
    }
}

fn evidence_id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
