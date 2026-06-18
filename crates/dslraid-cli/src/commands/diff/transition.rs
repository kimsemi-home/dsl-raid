mod added;
mod changed;
mod removed;

use super::model::{DiffChange, DiffSummary, DiffWarning};
use added::record_added_transition;
use changed::record_changed_transition;
use removed::record_removed_transition;
use std::collections::BTreeSet;

pub(super) fn record_transition_review(
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    change: &DiffChange,
    base_terminal_states: &BTreeSet<String>,
    head_terminal_states: &BTreeSet<String>,
) {
    match change.action {
        "added" => record_added_transition(summary, warnings, change, head_terminal_states),
        "removed" => record_removed_transition(summary, warnings, change, base_terminal_states),
        "changed" => record_changed_transition(
            summary,
            warnings,
            change,
            base_terminal_states,
            head_terminal_states,
        ),
        _ => {}
    }
}
