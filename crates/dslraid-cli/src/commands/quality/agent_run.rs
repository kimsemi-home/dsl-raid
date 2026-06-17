mod authority;
mod debt;
mod evidence;
mod fields;
mod lease;
mod lock_ref;
mod reviewer;

use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::{fs, path::Path};

pub(super) fn check(path: &Path, lock_path: &Path) -> Result<()> {
    let value: Value = serde_json::from_slice(
        &fs::read(path).with_context(|| format!("read {}", path.display()))?,
    )?;
    let lock: Value = serde_json::from_slice(
        &fs::read(lock_path).with_context(|| format!("read {}", lock_path.display()))?,
    )?;
    let issues = semantic_issues_with_lock(&value, &lock);
    if issues.is_empty() {
        println!("agent run semantic gate ok");
        return Ok(());
    }
    bail!("agent run semantic gate failed: {}", issues.join("; "))
}

#[cfg(test)]
pub(super) fn semantic_issues(value: &Value) -> Vec<String> {
    semantic_issues_with_optional_lock(value, None)
}

pub(super) fn semantic_issues_with_lock(value: &Value, lock: &Value) -> Vec<String> {
    semantic_issues_with_optional_lock(value, Some(lock))
}

fn semantic_issues_with_optional_lock(value: &Value, lock: Option<&Value>) -> Vec<String> {
    let mut issues = Vec::new();
    authority::push_verified_gate_issue(value, &mut issues);
    if !authority::is_approved(value) {
        return issues;
    }
    push_approved_issues(value, lock, &mut issues);
    issues
}

fn push_approved_issues(value: &Value, lock: Option<&Value>, issues: &mut Vec<String>) {
    authority::push_self_approval_issue(value, issues);
    lease::push_issues(value, issues);
    evidence::push_issues(value, issues);
    reviewer::push_issues(value, issues);
    debt::push_issues(value, issues);
    lock_ref::push_issues(value, lock, issues);
}
