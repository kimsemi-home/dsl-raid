mod relation;
mod target;

use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;
use std::collections::HashSet;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let ids = evidence_ids(value);
    if !ids.is_empty() && !has_link(value) {
        issues.push("approved run requires evidence graph link".to_string());
    }
    for evidence in active_items(value) {
        for link in items(evidence, "links") {
            relation::push_issue(evidence, link, issues);
            target::push_issue(&ids, evidence, link, issues);
        }
    }
}

fn has_link(value: &Value) -> bool {
    active_items(value).any(|evidence| items(evidence, "links").next().is_some())
}

fn evidence_ids(value: &Value) -> HashSet<&str> {
    items(value, "evidence")
        .filter_map(|item| field_text(item, "id"))
        .collect()
}

fn active_items(value: &Value) -> impl Iterator<Item = &Value> {
    items(value, "evidence").filter(|item| !field_is(item, "status", "pruned"))
}

fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
