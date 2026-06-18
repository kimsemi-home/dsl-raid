use crate::commands::quality::agent_run::fields::{field_is, field_text, items, text};
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
    if !version_matches(value, update) {
        issues.push(format!(
            "ssot defect claim {} requires current ontology knowledge update",
            claim_id
        ));
    }
}

fn linked_update<'a>(value: &'a Value, refs: &[&str]) -> Option<&'a Value> {
    items(value, "debts")
        .filter(is_closed_review)
        .filter(|debt| refs.iter().any(|reference| has_evidence(debt, reference)))
        .find_map(|debt| learning_update(debt, refs))
}

fn learning_update<'a>(value: &'a Value, refs: &[&str]) -> Option<&'a Value> {
    value
        .get("updates")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|update| is_applied_learning(update))
        .find(|update| refs.iter().any(|reference| has_evidence(update, reference)))
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

fn version_matches(value: &Value, update: &Value) -> bool {
    text(value, &["ssot", "ontology_version"])
        .is_some_and(|version| field_text(update, "ontology_version") == Some(version))
}

fn has_evidence(value: &Value, reference: &str) -> bool {
    value
        .get("evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .any(|item| item.as_str() == Some(reference))
}

fn refs(value: &Value) -> Vec<&str> {
    value
        .get("evidence")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}
