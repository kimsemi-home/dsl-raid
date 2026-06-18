use crate::commands::quality::agent_run::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn push_evidence_issue(value: &Value, issues: &mut Vec<String>) {
    if !is_governance_authority(value) {
        return;
    }
    let Some(approver) = text(value, &["authority_gate", "approved_by"]) else {
        return;
    };
    if approver.starts_with("steward:") && !has_approver_evidence(value, approver) {
        issues.push(format!(
            "governance steward approver {approver} requires authority evidence"
        ));
    }
}

fn is_governance_authority(value: &Value) -> bool {
    text(value, &["authority_gate", "scope"]) == Some("authority")
        && text(value, &["authority_gate", "profile"]) == Some("governance")
}

fn has_approver_evidence(value: &Value, approver: &str) -> bool {
    authority_refs(value).iter().any(|reference| {
        items(value, "evidence").any(|evidence| {
            field_text(evidence, "id") == Some(reference.as_str())
                && field_text(evidence, "subject") == Some(approver)
        })
    })
}

fn authority_refs(value: &Value) -> Vec<String> {
    value
        .pointer("/authority_gate/evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}
