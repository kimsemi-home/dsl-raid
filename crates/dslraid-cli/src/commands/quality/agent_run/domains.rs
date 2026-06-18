pub(super) mod agreement;
pub(super) mod approved;
pub(super) mod artifact;
pub(super) mod authority;
pub(super) mod claim;
pub(super) mod containment;
pub(super) mod debt;
pub(super) mod evidence;
pub(super) mod lease;
pub(super) mod orchestration;
pub(super) mod participants;
pub(super) mod refs;
pub(super) mod semantic_diff;
pub(super) mod ssot;
pub(super) mod translation;

use serde_json::Value;
use std::path::Path;

pub(super) use super::fields;

pub(super) fn semantic_issues(
    value: &Value,
    lock: Option<&Value>,
    root: Option<&Path>,
) -> Vec<String> {
    let mut issues = Vec::new();
    authority::push_gate_issues(value, &mut issues);
    containment::push_issues(value, &mut issues);
    lease::push_gate_issues(value, &mut issues);
    if authority::is_approved(value) {
        approved::push_issues(value, lock, root, &mut issues);
    }
    issues
}
