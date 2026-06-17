use super::id;
use crate::commands::quality::agent_run::fields::{field_is, field_text};
use serde_json::Value;

pub(super) fn push_issues(debt: &Value, issues: &mut Vec<String>) {
    if !field_is(debt, "kind", "review") {
        return;
    }
    let updates = updates(debt);
    if updates.is_empty() || updates.iter().any(is_learning_update) {
        return;
    }
    issues.push(format!(
        "debt {} requires policy, ontology, or spec knowledge update",
        id(debt)
    ));
}

fn is_learning_update(value: &&Value) -> bool {
    matches!(
        field_text(value, "kind"),
        Some("policy" | "ontology" | "spec")
    )
}

fn updates(value: &Value) -> Vec<&Value> {
    value
        .get("updates")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .collect()
}
