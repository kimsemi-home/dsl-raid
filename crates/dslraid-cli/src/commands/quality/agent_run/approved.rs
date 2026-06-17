use super::{
    agreement, artifact, authority, claim, coverage_ref, debt, evidence, evidence_quality, lease,
    lock_ref, review_capacity, reviewer, semantic_diff, ssot, trace_ref, translation,
};
use serde_json::Value;
use std::path::Path;

pub(super) fn push_issues(
    value: &Value,
    lock: Option<&Value>,
    root: Option<&Path>,
    issues: &mut Vec<String>,
) {
    authority::push_approved_issues(value, issues);
    ssot::push_issues(value, issues);
    lease::push_issues(value, issues);
    evidence::push_issues(value, issues);
    evidence_quality::push_issues(value, issues);
    reviewer::push_issues(value, issues);
    review_capacity::push_issues(value, issues);
    agreement::push_issues(value, issues);
    semantic_diff::push_issues(value, issues);
    artifact::push_issues(value, issues);
    claim::push_issues(value, issues);
    translation::push_issues(value, issues);
    debt::push_issues(value, issues);
    lock_ref::push_issues(value, lock, issues);
    trace_ref::push_issues(value, root, issues);
    coverage_ref::push_issues(value, root, issues);
}
