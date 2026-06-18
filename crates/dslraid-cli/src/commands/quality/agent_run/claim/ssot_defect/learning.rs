mod evidence;
mod impact;
mod lineage;
mod verification;
mod version;

use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    let refs = refs(claim);
    let claim_id = field_text(claim, "id").unwrap_or("<unknown>");
    let Some(update) = linked_update(value, &refs) else {
        issues.push(format!(
            "ssot defect claim {} requires linked knowledge update",
            claim_id
        ));
        return;
    };
    if !version::matches(value, update) {
        push(claim_id, "current ontology knowledge update", issues);
    }
    if !impact::covers(update, claim) {
        push(claim_id, "affected knowledge update subject", issues);
    }
    if !lineage::has_prior(update) {
        push(claim_id, "prior knowledge link", issues);
    }
    if !verification::matches(update, claim) {
        push(claim_id, "knowledge update verification plan", issues);
    }
}

fn linked_update<'a>(value: &'a Value, refs: &[&str]) -> Option<&'a Value> {
    items(value, "debts")
        .filter(is_closed_review)
        .filter(|debt| refs.iter().any(|reference| evidence::has(debt, reference)))
        .find_map(|debt| learning_update(debt, refs))
}

fn learning_update<'a>(value: &'a Value, refs: &[&str]) -> Option<&'a Value> {
    value
        .get("updates")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|update| is_applied_learning(update))
        .find(|update| {
            refs.iter()
                .any(|reference| evidence::has(update, reference))
        })
}

fn is_applied_learning(value: &Value) -> bool {
    field_is(value, "status", "applied")
        && matches!(
            field_text(value, "kind"),
            Some("policy" | "ontology" | "spec")
        )
}

fn is_closed_review(value: &&Value) -> bool {
    field_is(value, "kind", "review") && field_is(value, "status", "closed")
}

fn refs(value: &Value) -> Vec<&str> {
    evidence::ids(value)
}

fn push(claim_id: &str, requirement: &str, issues: &mut Vec<String>) {
    issues.push(format!(
        "ssot defect claim {claim_id} requires {requirement}"
    ));
}
