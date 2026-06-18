use super::model::{DiffChange, DiffWarning};

pub(super) fn sort_report_entries(changes: &mut [DiffChange], warnings: &mut [DiffWarning]) {
    changes.sort_by(|left, right| {
        (left.action, left.kind.as_str(), left.subject.as_str()).cmp(&(
            right.action,
            right.kind.as_str(),
            right.subject.as_str(),
        ))
    });
    warnings.sort_by(|left, right| {
        (left.code, left.subject.as_str()).cmp(&(right.code, right.subject.as_str()))
    });
}
