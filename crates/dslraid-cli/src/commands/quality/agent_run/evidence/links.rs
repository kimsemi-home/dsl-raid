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
            push_relation_issue(evidence, link, issues);
            push_target_issue(&ids, evidence, link, issues);
        }
    }
}

fn push_relation_issue(evidence: &Value, link: &Value, issues: &mut Vec<String>) {
    let Some(relation) = field_text(link, "relation") else {
        issues.push(format!("evidence {} link requires relation", id(evidence)));
        return;
    };
    if !matches!(
        relation,
        "corroborates" | "derived_from" | "reproduces" | "observed_with" | "supersedes"
    ) {
        issues.push(format!(
            "evidence {} has unsupported link relation {relation}",
            id(evidence)
        ));
    }
}

fn push_target_issue(
    ids: &HashSet<&str>,
    evidence: &Value,
    link: &Value,
    issues: &mut Vec<String>,
) {
    let Some(target) = field_text(link, "target") else {
        issues.push(format!("evidence {} link requires target", id(evidence)));
        return;
    };
    if target == id(evidence) {
        issues.push(format!(
            "evidence {} link cannot target itself",
            id(evidence)
        ));
    } else if !ids.contains(target) {
        issues.push(format!(
            "evidence {} link target {target} is not evidence",
            id(evidence)
        ));
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
