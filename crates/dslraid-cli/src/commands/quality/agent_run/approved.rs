use super::{
    agreement, artifact, authority, claim, debt, evidence, lease, orchestration, participants,
    refs, semantic_diff, ssot, translation,
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
    participants::push_producer_issues(value, issues);
    ssot::push_issues(value, issues);
    lease::push_issues(value, issues);
    evidence::push_issues(value, issues);
    evidence::push_quality_issues(value, issues);
    orchestration::push_issues(value, issues);
    participants::push_review_issues(value, issues);
    agreement::push_issues(value, issues);
    semantic_diff::push_issues(value, issues);
    artifact::push_issues(value, issues);
    claim::push_issues(value, issues);
    translation::push_issues(value, issues);
    debt::push_issues(value, issues);
    refs::push_issues(value, lock, root, issues);
}
