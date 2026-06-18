use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_refs(
    item: &Value,
    key: &str,
    known: &BTreeSet<String>,
    label: &str,
    issues: &mut Vec<String>,
) {
    if known.is_empty() {
        return;
    }
    let refs = refs(item, key);
    if refs.is_empty() {
        issues.push(format!("orchestration receipt requires {key}"));
    }
    for reference in refs {
        push_unknown(reference, known, label, issues);
    }
}

fn push_unknown(reference: &str, known: &BTreeSet<String>, label: &str, issues: &mut Vec<String>) {
    if !known.contains(reference) {
        issues.push(format!(
            "orchestration references unknown {label} {reference}"
        ));
    }
}

fn refs<'a>(value: &'a Value, key: &str) -> Vec<&'a str> {
    value
        .get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}
