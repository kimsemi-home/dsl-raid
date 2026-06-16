use super::model::DiffReport;

pub(super) fn render(report: &DiffReport) -> String {
    let mut lines = Vec::new();
    lines.push(format!("diff {}", report.status));
    lines.push(format!("base: {} {}", report.base.path, report.base.hash));
    lines.push(format!("head: {} {}", report.head.path, report.head.hash));
    lines.push(format!(
        "summary: +{} -{} ~{}",
        report.summary.added, report.summary.removed, report.summary.changed
    ));
    lines.push(format!(
        "fsm: states +{} -{} ~{} transitions +{} -{} ~{} terminal_paths ~{} untested_added {} policy_traces ~{}",
        report.summary.review.states_added,
        report.summary.review.states_removed,
        report.summary.review.states_changed,
        report.summary.review.transitions_added,
        report.summary.review.transitions_removed,
        report.summary.review.transitions_changed,
        report.summary.review.terminal_paths_changed,
        report.summary.review.untested_transitions_added,
        report.summary.review.policy_traces_changed
    ));
    push_warnings(report, &mut lines);
    push_changes(report, &mut lines);
    lines.join("\n")
}

fn push_warnings(report: &DiffReport, lines: &mut Vec<String>) {
    if report.warnings.is_empty() {
        return;
    }
    lines.push("warnings:".to_string());
    for warning in &report.warnings {
        lines.push(format!(
            "{} {} {}: {}",
            warning.severity, warning.code, warning.subject, warning.message
        ));
    }
}

fn push_changes(report: &DiffReport, lines: &mut Vec<String>) {
    if report.changes.is_empty() {
        return;
    }
    lines.push("changes:".to_string());
    for change in &report.changes {
        let sign = match change.action {
            "added" => "+",
            "removed" => "-",
            "changed" => "~",
            _ => "?",
        };
        let fields = if change.fields.is_empty() {
            String::new()
        } else {
            format!(" fields={}", change.fields.join(","))
        };
        lines.push(format!(
            "{sign} {} {} {}{}",
            change.kind, change.subject, change.label, fields
        ));
    }
}
