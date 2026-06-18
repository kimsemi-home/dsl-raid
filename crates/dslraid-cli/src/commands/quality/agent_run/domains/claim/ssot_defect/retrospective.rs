use crate::commands::quality::agent_run::fields::{field_is, items};
use serde_json::Value;

pub(super) fn has_linked_review_debt(value: &Value, claim: &Value) -> bool {
    let refs = refs(claim);
    !refs.is_empty()
        && items(value, "debts")
            .filter(is_closed_review)
            .any(|debt| refs.iter().any(|reference| has_evidence(debt, reference)))
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
