mod sensitive;

use super::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    push_required_issue(value, "reasoning_level", "reasoning level", issues);
    push_required_issue(value, "trust_tier", "trust tier", issues);
    push_cold_start_issue(value, issues);
    push_automatic_issue(value, issues);
    push_high_risk_issue(value, issues);
    sensitive::push_issues(value, issues);
}

fn push_required_issue(value: &Value, key: &str, label: &str, issues: &mut Vec<String>) {
    if text(value, &["producer", key]).is_none() {
        issues.push(format!("approved run requires producer {label}"));
    }
}

fn push_cold_start_issue(value: &Value, issues: &mut Vec<String>) {
    if matches!(trust(value), Some("T0" | "T1")) {
        issues.push(format!(
            "approved run cannot use cold-start producer {}",
            id(value)
        ));
    }
}

fn push_automatic_issue(value: &Value, issues: &mut Vec<String>) {
    if text(value, &["authority_gate", "profile"]) != Some("automatic") {
        return;
    }
    if !matches!(trust(value), Some("T3" | "T4")) {
        issues.push("automatic authority requires trusted producer T3 or T4".to_string());
    }
}

fn push_high_risk_issue(value: &Value, issues: &mut Vec<String>) {
    if !matches!(
        scope(value),
        Some("security" | "audit" | "ontology" | "incident" | "authority")
    ) {
        return;
    }
    if !matches!(reasoning(value), Some("R3" | "R4")) {
        issues.push("high-risk authority requires producer reasoning level R3 or R4".to_string());
    }
}

fn id(value: &Value) -> &str {
    text(value, &["producer", "id"]).unwrap_or("<unknown>")
}

fn reasoning(value: &Value) -> Option<&str> {
    text(value, &["producer", "reasoning_level"])
}

fn scope(value: &Value) -> Option<&str> {
    text(value, &["authority_gate", "scope"])
}

fn trust(value: &Value) -> Option<&str> {
    text(value, &["producer", "trust_tier"])
}
