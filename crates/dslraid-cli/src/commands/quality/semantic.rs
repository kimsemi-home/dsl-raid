use anyhow::{bail, Result};
use dslraid_core::{load_core_ir, CoreIr};
use std::path::Path;

pub(super) fn check(input: &Path) -> Result<CoreIr> {
    let ir = load_core_ir(input)?;
    let report = crate::validation_report(&ir, input, "quality", Vec::new())?;
    if !report.is_success(&[]) {
        crate::print_report_text(&report);
        bail!("semantic quality failed");
    }
    Ok(ir)
}
