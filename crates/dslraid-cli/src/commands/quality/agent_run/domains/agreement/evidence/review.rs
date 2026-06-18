use super::super::id;
use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issue(
    root: &Value,
    agreement: &Value,
    refs: &[&str],
    evidence: &BTreeSet<String>,
    issues: &mut Vec<String>,
) {
    let refs = known_refs(refs, evidence);
    if refs.is_empty() {
        return;
    }
    if refs.iter().all(|reference| is_review(root, reference)) {
        issues.push(format!(
            "agreement {} cannot rely only on review evidence",
            id(agreement)
        ));
    }
}

fn known_refs<'a>(refs: &'a [&str], evidence: &BTreeSet<String>) -> Vec<&'a str> {
    refs.iter()
        .copied()
        .filter(|reference| evidence.contains(*reference))
        .collect()
}

fn is_review(root: &Value, id: &str) -> bool {
    items(root, "evidence").any(|item| field_text(item, "id") == Some(id) && is_kind(item))
}

fn is_kind(value: &Value) -> bool {
    field_is(value, "kind", "review")
}
