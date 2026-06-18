use super::model::{DiffChange, DiffSummary, DiffWarning};
use super::scan_changed::record_changed;
use super::scan_removed::record_removed;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn record_removed_and_changed(
    base_items: &BTreeMap<String, Value>,
    head_items: &BTreeMap<String, Value>,
    base_terminal_states: &BTreeSet<String>,
    head_terminal_states: &BTreeSet<String>,
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    changes: &mut Vec<DiffChange>,
) {
    record_removed(
        base_items,
        head_items,
        base_terminal_states,
        head_terminal_states,
        summary,
        warnings,
        changes,
    );
    record_changed(
        base_items,
        head_items,
        base_terminal_states,
        head_terminal_states,
        summary,
        warnings,
        changes,
    );
}
