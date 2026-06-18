use serde_json::{json, Value};
use std::collections::BTreeSet;

pub(super) fn push_if_missing(
    subjects: &BTreeSet<String>,
    missing: &mut Vec<Value>,
    source: &str,
    reference: &str,
) {
    if !subjects.contains(reference) {
        missing.push(json!({ "source": source, "reference": reference }));
    }
}
