use super::fields::{changed_fields, item_string};
use super::model::{DiffChange, DiffSummary, DiffWarning};
use super::record::record_diff_change;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn record_changed(
    base_items: &BTreeMap<String, Value>,
    head_items: &BTreeMap<String, Value>,
    base_terminal_states: &BTreeSet<String>,
    head_terminal_states: &BTreeSet<String>,
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    changes: &mut Vec<DiffChange>,
) {
    for subject in base_items
        .keys()
        .filter(|subject| head_items.contains_key(*subject))
    {
        let before = base_items.get(subject).expect("subject came from base");
        let after = head_items.get(subject).expect("subject came from head");
        if before == after {
            continue;
        }
        let change = DiffChange {
            action: "changed",
            kind: item_string(after, "kind"),
            subject: subject.clone(),
            label: item_string(after, "label"),
            fields: changed_fields(before, after),
            before: Some(before.clone()),
            after: Some(after.clone()),
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
