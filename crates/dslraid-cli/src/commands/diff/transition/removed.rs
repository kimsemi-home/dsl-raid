use super::super::fields::has_policy_trace;
use super::super::model::{DiffChange, DiffSummary, DiffWarning};
use super::super::terminal::transition_points_to_terminal;
use super::super::warning::diff_warning;
use std::collections::BTreeSet;

pub(super) fn record_removed_transition(
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    change: &DiffChange,
    base_terminal_states: &BTreeSet<String>,
) {
    summary.review.transitions_removed += 1;
    if change
        .before
        .as_ref()
        .is_some_and(|item| transition_points_to_terminal(item, base_terminal_states))
    {
        summary.review.terminal_paths_changed += 1;
        warnings.push(diff_warning(
            "DIF021",
            &change.subject,
            "transition removes a terminal path",
        ));
    }
    if change.before.as_ref().is_some_and(has_policy_trace) {
        summary.review.policy_traces_changed += 1;
        warnings.push(diff_warning(
            "DIF030",
            &change.subject,
            "transition policy requirements removed",
        ));
    }
}
