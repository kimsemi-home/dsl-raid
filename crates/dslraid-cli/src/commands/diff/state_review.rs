use super::fields::is_terminal_state_item;
use super::model::{DiffChange, DiffSummary, DiffWarning};
use super::warning::diff_warning;

pub(super) fn record_state_review(
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    change: &DiffChange,
) {
    match change.action {
        "added" => {
            summary.review.states_added += 1;
            if change.after.as_ref().is_some_and(is_terminal_state_item) {
                summary.review.terminal_states_added += 1;
                warnings.push(diff_warning(
                    "DIF020",
                    &change.subject,
                    "terminal state added; review completion and failure paths",
                ));
            }
        }
        "removed" => {
            summary.review.states_removed += 1;
            if change.before.as_ref().is_some_and(is_terminal_state_item) {
                summary.review.terminal_states_removed += 1;
                warnings.push(diff_warning(
                    "DIF020",
                    &change.subject,
                    "terminal state removed; review completion and failure paths",
                ));
            }
        }
        "changed" => record_changed_state(summary, warnings, change),
        _ => {}
    }
}

fn record_changed_state(
    summary: &mut DiffSummary,
    warnings: &mut Vec<DiffWarning>,
    change: &DiffChange,
) {
    summary.review.states_changed += 1;
    if change
        .fields
        .iter()
        .any(|field| field == "terminal" || field == "terminal_semantics")
    {
        warnings.push(diff_warning(
            "DIF020",
            &change.subject,
            "terminal state semantics changed",
        ));
    }
}
