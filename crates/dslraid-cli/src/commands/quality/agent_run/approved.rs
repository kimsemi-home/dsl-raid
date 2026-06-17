use super::{
    authority, coverage_ref, debt, evidence, evidence_quality, lease, lock_ref, reviewer, ssot,
    trace_ref, translation,
};
use serde_json::Value;
use std::path::Path;

pub(super) fn push_issues(
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
    translation::push_issues(value, issues);
    debt::push_issues(value, issues);
    lock_ref::push_issues(value, lock, issues);
    trace_ref::push_issues(value, root, issues);
    coverage_ref::push_issues(value, root, issues);
}
