mod fields;

use anyhow::{bail, Context, Result};
use fields::{field_is, items, text};
use serde_json::Value;
use std::{fs, path::Path};

pub(super) fn check(path: &Path) -> Result<()> {
    let value: Value = serde_json::from_slice(
        &fs::read(path).with_context(|| format!("read {}", path.display()))?,
    )?;
    let issues = semantic_issues(&value);
    if issues.is_empty() {
        println!("agent run semantic gate ok");
        return Ok(());
    }
    bail!("agent run semantic gate failed: {}", issues.join("; "))
}

pub(super) fn semantic_issues(value: &Value) -> Vec<String> {
    let mut issues = Vec::new();
    if text(value, &["run", "status"]) == Some("verified")
        && text(value, &["authority_gate", "decision"]) != Some("approved")
    {
        issues.push("verified run requires approved authority gate".to_string());
    }
    if text(value, &["authority_gate", "decision"]) != Some("approved") {
        return issues;
    }
    push_approved_issues(value, &mut issues);
    issues
}

fn push_approved_issues(value: &Value, issues: &mut Vec<String>) {
    let producer = text(value, &["producer", "id"]);
    if text(value, &["authority_gate", "approved_by"]) == producer {
        issues.push("producer cannot approve its own output".to_string());
    }
    if text(value, &["lease", "status"]) != Some("finished") {
        issues.push("approved run requires finished lease".to_string());
    }
    if !has_high_quality_evidence(value) {
        issues.push("approved run requires high quality evidence".to_string());
    }
    if independent_reviewers(value, producer) == 0 {
        issues.push("approved run requires independent reviewer".to_string());
    }
    if has_open_debt(value) {
        issues.push("approved run cannot carry open debt".to_string());
    }
}

fn has_high_quality_evidence(value: &Value) -> bool {
    items(value, "evidence").any(|item| field_is(item, "quality", "high"))
}

fn independent_reviewers(value: &Value, producer: Option<&str>) -> usize {
    items(value, "reviewers")
        .filter(|item| item.get("id").and_then(Value::as_str) != producer)
        .count()
}

fn has_open_debt(value: &Value) -> bool {
    items(value, "debts").any(|item| field_is(item, "status", "open"))
}
