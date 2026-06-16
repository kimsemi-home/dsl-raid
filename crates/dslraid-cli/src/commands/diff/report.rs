use super::fields::item_string;
use super::model::{DiffChange, DiffEndpoint, DiffReport, DiffSummary};
use super::record::record_diff_change;
use super::scan::record_removed_and_changed;
use super::terminal::terminal_state_subjects;
use anyhow::Result;
use dslraid_core::{sha256_json, CoreIr};
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

    for subject in head_items
        .keys()
        .filter(|subject| !base_items.contains_key(*subject))
    {
        let after = head_items
            .get(subject)
            .expect("subject came from head")
            .clone();
        let change = DiffChange {
            action: "added",
            kind: item_string(&after, "kind"),
            subject: subject.clone(),
            label: item_string(&after, "label"),
            fields: Vec::new(),
            before: None,
            after: Some(after),
        };
        record_diff_change(
            &mut summary,
            &mut warnings,
            &change,
            &base_terminal_states,
            &head_terminal_states,
        );
        changes.push(change);
    }

    record_removed_and_changed(
        &base_items,
        &head_items,
        &base_terminal_states,
        &head_terminal_states,
        &mut summary,
        &mut warnings,
        &mut changes,
    );
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

fn endpoint(path: &Path, ir: &CoreIr) -> Result<DiffEndpoint> {
    Ok(DiffEndpoint {
        path: path.display().to_string(),
        hash: sha256_json(ir)?,
        ir_version: ir.ir_version.clone(),
    })
}
