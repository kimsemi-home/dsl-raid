use super::super::model::{DiffChange, DiffSummary, DiffWarning};
use super::super::terminal::transition_terminal_path_changed;
use super::super::warning::diff_warning;
use std::collections::BTreeSet;

pub(super) fn record_changed_transition(
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    change: &DiffChange,
    base_terminal_states: &BTreeSet<String>,
    head_terminal_states: &BTreeSet<String>,
) {
    summary.review.transitions_changed += 1;
    if transition_terminal_path_changed(change, base_terminal_states, head_terminal_states) {
        summary.review.terminal_paths_changed += 1;
        warnings.push(diff_warning(
            "DIF021",
            &change.subject,
            "transition terminal path changed",
        ));
    }
    if change.fields.iter().any(|field| field == "requires") {
        summary.review.policy_traces_changed += 1;
        warnings.push(diff_warning(
            "DIF030",
            &change.subject,
            "transition policy requirements changed",
        ));
    }
}
