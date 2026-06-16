use super::model::{DiffChange, DiffSummary, DiffWarning};
use super::state_review::record_state_review;
use super::transition_review::record_transition_review;
use std::collections::BTreeSet;

pub(super) fn record_diff_change(
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    change: &DiffChange,
    base_terminal_states: &BTreeSet<String>,
    head_terminal_states: &BTreeSet<String>,
) {
    match change.action {
        "added" => summary.added += 1,
        "removed" => summary.removed += 1,
        "changed" => summary.changed += 1,
        _ => {}
    }
    let kind_summary = summary.by_kind.entry(change.kind.clone()).or_default();
    match change.action {
        "added" => kind_summary.added += 1,
        "removed" => kind_summary.removed += 1,
        "changed" => kind_summary.changed += 1,
        _ => {}
    }

    match change.kind.as_str() {
        "state" => record_state_review(summary, warnings, change),
        "transition" => record_transition_review(
            summary,
            warnings,
            change,
            base_terminal_states,
            head_terminal_states,
        ),
        _ => {}
    }
}
