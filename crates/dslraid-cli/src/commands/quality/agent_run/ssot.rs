use super::fields::{field_text, items, text};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    push_versioned_authority_issues(value, issues);
    let status = text(value, &["ssot", "revalidation", "status"]);
    if status.is_none() {
        issues.push("approved run requires ssot revalidation status".to_string());
        return;
    }
    if status.is_some_and(blocks_authority) {
        issues.push(format!(
            "approved run cannot use ssot revalidation status {}",
            status.unwrap()
        ));
    }
    for field in ["assessed_at", "assessor", "revalidate_at"] {
        if text(value, &["ssot", "revalidation", field]).is_none() {
            issues.push(format!("ssot revalidation requires {field}"));
        }
    }
    push_evidence_issue(value, issues);
}

fn push_versioned_authority_issues(value: &Value, issues: &mut Vec<String>) {
    for field in ["context", "ontology_version", "contract_version"] {
        if text(value, &["ssot", field]).is_none() {
            issues.push(format!("approved run requires ssot {field}"));
        }
    }
}

fn blocks_authority(status: &str) -> bool {
    matches!(status, "expired" | "frozen" | "superseded" | "retired")
}

fn push_evidence_issue(value: &Value, issues: &mut Vec<String>) {
    let Some(reference) = text(value, &["ssot", "revalidation", "evidence"]) else {
        return;
    };
    if !evidence_ids(value).contains(reference) {
        issues.push(format!(
            "ssot revalidation references unknown evidence {reference}"
        ));
    }
}

fn evidence_ids(value: &Value) -> BTreeSet<String> {
    items(value, "evidence")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}
