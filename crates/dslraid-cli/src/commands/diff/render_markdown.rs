use super::model::DiffReport;

pub(super) fn render(report: &DiffReport) -> String {
    let mut lines = Vec::new();
    lines.push("# DSLRaid Diff".to_string());
    lines.push(String::new());
    lines.push(format!("Status: **{}**", report.status));
    lines.push(format!(
        "- Base: `{}` `{}`",
        report.base.path, report.base.hash
    ));
    lines.push(format!(
        "- Head: `{}` `{}`",
        report.head.path, report.head.hash
    ));
    lines.push(String::new());
    push_summary(report, &mut lines);
    push_warnings(report, &mut lines);
    push_changes(report, &mut lines);
    lines.join("\n")
}

fn push_summary(report: &DiffReport, lines: &mut Vec<String>) {
    lines.push("## Summary".to_string());
    lines.push(format!("- Added: {}", report.summary.added));
    lines.push(format!("- Removed: {}", report.summary.removed));
    lines.push(format!("- Changed: {}", report.summary.changed));
    lines.push(format!(
        "- FSM states: +{} -{} ~{}",
        report.summary.review.states_added,
        report.summary.review.states_removed,
        report.summary.review.states_changed
    ));
    lines.push(format!(
        "- FSM transitions: +{} -{} ~{}",
        report.summary.review.transitions_added,
        report.summary.review.transitions_removed,
        report.summary.review.transitions_changed
    ));
    lines.push(format!(
        "- Review flags: terminal paths ~{}, untested transitions +{}, policy traces ~{}",
        report.summary.review.terminal_paths_changed,
        report.summary.review.untested_transitions_added,
        report.summary.review.policy_traces_changed
    ));
    lines.push(String::new());
}

fn push_warnings(report: &DiffReport, lines: &mut Vec<String>) {
    lines.push("## Warnings".to_string());
    if report.warnings.is_empty() {
        lines.push("- none".to_string());
    } else {
        for warning in &report.warnings {
            lines.push(format!(
                "- {} `{}` `{}`: {}",
                warning.severity, warning.code, warning.subject, warning.message
            ));
        }
    }
    lines.push(String::new());
}

fn push_changes(report: &DiffReport, lines: &mut Vec<String>) {
    lines.push("## Changes".to_string());
    if report.changes.is_empty() {
        lines.push("- none".to_string());
        return;
    }
    lines.push("| Action | Kind | Subject | Fields |".to_string());
    lines.push("| --- | --- | --- | --- |".to_string());
    for change in &report.changes {
        let fields = if change.fields.is_empty() {
            "-".to_string()
        } else {
            change.fields.join(", ")
        };
        lines.push(format!(
            "| {} | {} | `{}` | {} |",
            change.action, change.kind, change.subject, fields
        ));
    }
}
