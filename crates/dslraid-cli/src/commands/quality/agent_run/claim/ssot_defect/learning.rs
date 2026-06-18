use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn has_linked_update(value: &Value, claim: &Value) -> bool {
    let refs = refs(claim);
    !refs.is_empty()
        && items(value, "debts")
            .filter(is_closed_review)
            .filter(|debt| refs.iter().any(|reference| has_evidence(debt, reference)))
            .any(|debt| has_learning_update(debt, &refs))
}

fn has_learning_update(value: &Value, refs: &[&str]) -> bool {
    value
        .get("updates")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|update| is_applied_learning(update))
        .any(|update| refs.iter().any(|reference| has_evidence(update, reference)))
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
