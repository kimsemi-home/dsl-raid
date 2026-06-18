mod condition;

use super::id;
use crate::commands::quality::agent_run::fields::{field_is, field_text};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_issues(item: &Value, evidence_ids: &BTreeSet<String>, issues: &mut Vec<String>) {
    if !field_is(item, "status", "released") {
        return;
    }
    push_releaser_issue(item, issues);
    condition::push_issues(item, evidence_ids, issues);
}

fn push_releaser_issue(item: &Value, issues: &mut Vec<String>) {
    let releaser = field_text(item, "released_by").unwrap_or("");
    if field_is(item, "kind", "quarantine") && !releaser.starts_with("steward:") {
        issues.push(format!(
            "released quarantine {} requires steward release",
            id(item)
        ));
    }
}
