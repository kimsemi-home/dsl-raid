use super::evidence;
use crate::commands::quality::agent_run::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn linked_update<'a>(value: &'a Value, claim: &Value) -> Option<&'a Value> {
    let refs = evidence::ids(claim);
    items(value, "debts")
        .filter(is_closed_review)
        .filter(|debt| refs.iter().any(|reference| evidence::has(debt, reference)))
        .find_map(|debt| learning_update(debt, &refs))
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
