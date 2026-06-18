mod markdown;
mod sections;
mod text;

use super::model::DiffReport;

pub(super) fn markdown_report(report: &DiffReport) -> String {
    markdown::render(report)
}

pub(super) fn text_report(report: &DiffReport) -> String {
    text::render(report)
}
