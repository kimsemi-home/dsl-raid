use super::overlay::coverage_overlay_value;
use crate::{validate_json_file, write_bytes, write_or_stdout};
use anyhow::Result;
use dslraid_core::load_core_ir;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub(crate) fn build(trace: &Path, design_ir: &Path, out: Option<&Path>) -> Result<()> {
    validate_json_file(Path::new("schemas/dslraid-trace.schema.json"), trace)?;
    let ir = load_core_ir(design_ir)?;
    let trace_value: Value = serde_json::from_slice(&fs::read(trace)?)?;
    let coverage = coverage_overlay_value(&ir, design_ir, trace, &trace_value)?;
    let temp_path = std::env::temp_dir().join(format!(
        "dslraid-coverage-build-{}.json",
        std::process::id()
    ));

    write_bytes(
        &temp_path,
        serde_json::to_string_pretty(&coverage)?.as_bytes(),
    )?;
    validate_json_file(
        Path::new("schemas/dslraid-coverage.schema.json"),
        &temp_path,
    )?;
    fs::remove_file(&temp_path).ok();

    write_or_stdout(out, serde_json::to_string_pretty(&coverage)?.as_bytes())
}
