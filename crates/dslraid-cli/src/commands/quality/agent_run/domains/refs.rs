mod coverage;
mod lock;
mod trace;

use serde_json::Value;
use std::path::Path;

pub(super) fn push_issues(
    value: &Value,
    lock: Option<&Value>,
    root: Option<&Path>,
    issues: &mut Vec<String>,
) {
    lock::push_issues(value, lock, issues);
    trace::push_issues(value, root, issues);
    coverage::push_issues(value, root, issues);
}
