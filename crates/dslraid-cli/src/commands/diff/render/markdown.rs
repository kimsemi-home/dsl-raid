use super::super::model::DiffReport;
use super::sections::{push_changes, push_summary, push_warnings};

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
