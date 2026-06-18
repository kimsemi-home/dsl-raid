use super::super::fields::has_policy_trace;
use super::super::model::{DiffChange, DiffSummary, DiffWarning};
use super::super::terminal::transition_points_to_terminal;
use super::super::warning::diff_warning;
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn record_added_transition(
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    change: &DiffChange,
    head_terminal_states: &BTreeSet<String>,
) {
    summary.review.transitions_added += 1;
    if change
        .after
        .as_ref()
        .is_some_and(|item| item.get("tested").and_then(Value::as_bool) == Some(false))
    {
        summary.review.untested_transitions_added += 1;
        warnings.push(diff_warning(
            "DIF010",
            &change.subject,
            "untested transition added",
        ));
    }
    record_terminal_and_policy_add(summary, warnings, change, head_terminal_states);
}

fn record_terminal_and_policy_add(
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    change: &DiffChange,
    head_terminal_states: &BTreeSet<String>,
) {
    if change
        .after
        .as_ref()
        .is_some_and(|item| transition_points_to_terminal(item, head_terminal_states))
    {
        summary.review.terminal_paths_changed += 1;
        warnings.push(diff_warning(
            "DIF021",
            &change.subject,
            "transition adds a terminal path",
        ));
    }
    if change.after.as_ref().is_some_and(has_policy_trace) {
        summary.review.policy_traces_changed += 1;
        warnings.push(diff_warning(
            "DIF030",
            &change.subject,
            "transition policy requirements added",
        ));
    }
}
