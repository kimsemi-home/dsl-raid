use super::added::record_added;
use super::endpoint::endpoint;
use super::model::{DiffReport, DiffSummary};
use super::order::sort_report_entries;
use super::scan::record_removed_and_changed;
use super::terminal::terminal_state_subjects;
use anyhow::Result;
use dslraid_core::CoreIr;
use std::path::Path;

pub(crate) fn report(
    base_ir: &CoreIr,
    head_ir: &CoreIr,
    base_path: &Path,
    head_path: &Path,
) -> Result<DiffReport> {
    let base_items = crate::commands::query::item_map(base_ir);
    let head_items = crate::commands::query::item_map(head_ir);
    let base_terminal_states = terminal_state_subjects(&base_items);
    let head_terminal_states = terminal_state_subjects(&head_items);
    let mut changes = Vec::new();
    let mut warnings = Vec::new();
    let mut summary = DiffSummary::default();

    record_added(
        &base_items,
        &head_items,
        &base_terminal_states,
        &head_terminal_states,
        &mut summary,
        &mut warnings,
        &mut changes,
    );
    record_removed_and_changed(
        &base_items,
        &head_items,
        &base_terminal_states,
        &head_terminal_states,
        &mut summary,
        &mut warnings,
        &mut changes,
    );
    sort_report_entries(&mut changes, &mut warnings);
    Ok(DiffReport {
        diff_version: "0.1.0",
        status: if changes.is_empty() && warnings.is_empty() {
            "unchanged"
        } else {
            "changed"
        },
        base: endpoint(base_path, base_ir)?,
        head: endpoint(head_path, head_ir)?,
        summary,
        changes,
        warnings,
    })
}
