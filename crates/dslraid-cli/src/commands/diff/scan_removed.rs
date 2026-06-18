use super::fields::item_string;
use super::model::{DiffChange, DiffSummary, DiffWarning};
use super::record::record_diff_change;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn record_removed(
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
        .filter(|subject| !head_items.contains_key(*subject))
    {
        record_removed_subject(
            subject,
            base_items,
            base_terminal_states,
            head_terminal_states,
            summary,
            warnings,
            changes,
        );
    }
}

fn record_removed_subject(
    subject: &str,
    base_items: &BTreeMap<String, Value>,
    base_terminal_states: &BTreeSet<String>,
    head_terminal_states: &BTreeSet<String>,
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    changes: &mut Vec<DiffChange>,
) {
    let before = base_items
        .get(subject)
        .expect("subject came from base")
        .clone();
    let change = DiffChange {
        action: "removed",
        kind: item_string(&before, "kind"),
        subject: subject.to_string(),
        label: item_string(&before, "label"),
        fields: Vec::new(),
        before: Some(before),
        after: None,
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
