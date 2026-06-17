mod authority;
mod coverage_ref;
mod debt;
mod evidence;
mod evidence_quality;
mod fields;
mod lease;
mod lock_ref;
mod reviewer;
mod ssot;
mod trace_ref;

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
    let issues = semantic_issues_with_context(&value, &lock, Path::new("."));
    if issues.is_empty() {
        println!("agent run semantic gate ok");
        return Ok(());
    }
    bail!("agent run semantic gate failed: {}", issues.join("; "))
}

#[cfg(test)]
pub(super) fn semantic_issues(value: &Value) -> Vec<String> {
    semantic_issues_with_optional_context(value, None, None)
}

pub(super) fn semantic_issues_with_context(
    value: &Value,
    lock: &Value,
    root: &Path,
) -> Vec<String> {
    semantic_issues_with_optional_context(value, Some(lock), Some(root))
}

fn semantic_issues_with_optional_context(
    value: &Value,
    lock: Option<&Value>,
    root: Option<&Path>,
) -> Vec<String> {
    let mut issues = Vec::new();
    authority::push_verified_gate_issue(value, &mut issues);
    if !authority::is_approved(value) {
        return issues;
    }
    push_approved_issues(value, lock, root, &mut issues);
    issues
}

fn push_approved_issues(
    value: &Value,
    lock: Option<&Value>,
    root: Option<&Path>,
    issues: &mut Vec<String>,
) {
    authority::push_self_approval_issue(value, issues);
    ssot::push_issues(value, issues);
    lease::push_issues(value, issues);
    evidence::push_issues(value, issues);
    evidence_quality::push_issues(value, issues);
    reviewer::push_issues(value, issues);
    debt::push_issues(value, issues);
    lock_ref::push_issues(value, lock, issues);
    trace_ref::push_issues(value, root, issues);
    coverage_ref::push_issues(value, root, issues);
}
