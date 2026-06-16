use anyhow::Result;
use dslraid_core::CoreIr;
use std::path::Path;

pub(super) fn generate(input: &Path, ir: &CoreIr, out: &Path) -> Result<()> {
    let report = crate::validation_report(ir, input, "validate", Vec::new())?;
    let mut bytes = serde_json::to_vec_pretty(&report)?;
    bytes.push(b'\n');
    crate::write_bytes(out, &bytes)
}
