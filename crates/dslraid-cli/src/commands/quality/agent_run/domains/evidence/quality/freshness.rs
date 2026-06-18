use super::super::super::fields::{field_is, field_text};
use serde_json::Value;

pub(super) fn push_issues(evidence: &Value, snapshots: &[&Value], issues: &mut Vec<String>) {
    for snapshot in snapshots
        .iter()
        .filter(|item| field_is(item, "quality", "high"))
    {
        push_snapshot_issue(evidence_id(evidence), snapshot, issues);
    }
}

fn push_snapshot_issue(id: &str, snapshot: &Value, issues: &mut Vec<String>) {
    if field_text(snapshot, "revalidate_at").is_some() {
        return;
    }
    issues.push(format!(
        "evidence {id} high quality snapshot requires revalidate_at"
    ));
}

fn evidence_id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
