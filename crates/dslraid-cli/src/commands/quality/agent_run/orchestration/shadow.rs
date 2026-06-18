use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    let Some(shadow) = item.get("shadow") else {
        issues.push("orchestration receipt requires shadow".to_string());
        return;
    };
    push_orchestrator_issue(item, shadow, issues);
    push_severity_issue(value, shadow, issues);
    push_evidence_issues(value, shadow, issues);
}

fn push_orchestrator_issue(item: &Value, shadow: &Value, issues: &mut Vec<String>) {
    let Some(primary) = field_text(item, "routed_by") else {
        return;
    };
    let Some(shadow_by) = field_text(shadow, "routed_by") else {
        issues.push("shadow orchestration requires routed_by".to_string());
        return;
    };
    if shadow_by == primary {
        issues.push(format!("shadow orchestrator cannot be primary {primary}"));
    }
}

fn push_severity_issue(value: &Value, shadow: &Value, issues: &mut Vec<String>) {
    let Some(severity) = field_text(shadow, "severity") else {
        issues.push("shadow orchestration requires severity".to_string());
        return;
    };
    if !matches!(severity, "D0" | "D1" | "D2" | "D3" | "D4") {
        issues.push(format!(
            "shadow orchestration has unsupported severity {severity}"
        ));
    }
    if matches!(severity, "D3" | "D4") && !human_review_required(value) {
        issues.push(format!("shadow severity {severity} requires human review"));
    }
}

fn human_review_required(value: &Value) -> bool {
    value
        .pointer("/authority_gate/human_review_required")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn push_evidence_issues(value: &Value, shadow: &Value, issues: &mut Vec<String>) {
    let refs: Vec<_> = items(shadow, "evidence")
        .filter_map(Value::as_str)
        .collect();
    if refs.is_empty() {
        issues.push("shadow orchestration requires evidence".to_string());
    }
    let known = evidence_ids(value);
    for reference in refs {
        if !known.contains(reference) {
            issues.push(format!(
                "shadow orchestration references unknown evidence {reference}"
            ));
        }
    }
}

fn evidence_ids(value: &Value) -> BTreeSet<String> {
    items(value, "evidence")
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}
