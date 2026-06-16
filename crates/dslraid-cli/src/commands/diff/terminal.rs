use super::model::DiffChange;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn terminal_state_subjects(items: &BTreeMap<String, Value>) -> BTreeSet<String> {
    items
        .iter()
        .filter(|(_, item)| {
            item.get("kind").and_then(Value::as_str) == Some("state")
                && item
                    .get("terminal")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
        })
        .map(|(subject, _)| subject.clone())
        .collect()
}

pub(super) fn transition_points_to_terminal(
    item: &Value,
    terminal_states: &BTreeSet<String>,
) -> bool {
    item.get("to")
        .and_then(Value::as_str)
        .is_some_and(|state| terminal_states.contains(state))
}

pub(super) fn transition_terminal_path_changed(
    change: &DiffChange,
    base_terminal_states: &BTreeSet<String>,
    head_terminal_states: &BTreeSet<String>,
) -> bool {
    let before_terminal = change
        .before
        .as_ref()
        .is_some_and(|item| transition_points_to_terminal(item, base_terminal_states));
    let after_terminal = change
        .after
        .as_ref()
        .is_some_and(|item| transition_points_to_terminal(item, head_terminal_states));
    before_terminal != after_terminal || change.fields.iter().any(|field| field == "to")
}
