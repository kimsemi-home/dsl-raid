use super::fields::item_string;
use super::model::{DiffChange, DiffSummary, DiffWarning};
use super::record::record_diff_change;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn record_added(
    base_items: &BTreeMap<String, Value>,
    head_items: &BTreeMap<String, Value>,
    base_terminal_states: &BTreeSet<String>,
    head_terminal_states: &BTreeSet<String>,
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    changes: &mut Vec<DiffChange>,
) {
    for subject in head_items
        .keys()
        .filter(|subject| !base_items.contains_key(*subject))
    {
        let after = head_items
            .get(subject)
            .expect("subject came from head")
            .clone();
        let change = DiffChange {
            action: "added",
            kind: item_string(&after, "kind"),
            subject: subject.clone(),
            label: item_string(&after, "label"),
            fields: Vec::new(),
            before: None,
            after: Some(after),
        };
        record_diff_change(
            summary,
            warnings,
            &change,
            base_terminal_states,
            head_terminal_states,
        );
        changes.push(change);
    }
}
