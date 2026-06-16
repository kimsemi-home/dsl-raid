mod counter;
mod design;
mod events;
mod missing;
mod output;
mod overlay;
mod seed;
mod subject;
mod trace_status;
mod value;

#[cfg(test)]
mod tests;

use crate::{validate_json_file, write_bytes, write_or_stdout, OutputFormat};
use anyhow::{bail, Result};
use dslraid_core::{load_core_ir, validate_json_schema};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub(crate) fn build(trace: &Path, design_ir: &Path, out: Option<&Path>) -> Result<()> {
    validate_json_file(Path::new("schemas/dslraid-trace.schema.json"), trace)?;
    let ir = load_core_ir(design_ir)?;
    let trace_value: Value = serde_json::from_slice(&fs::read(trace)?)?;
    let coverage = overlay::coverage_overlay_value(&ir, design_ir, trace, &trace_value)?;
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

pub(crate) fn check(coverage: &Path, design_ir: &Path, format: OutputFormat) -> Result<()> {
    let schema_issues =
        validate_json_schema(Path::new("schemas/dslraid-coverage.schema.json"), coverage)?;
    if !schema_issues.is_empty() {
        match format {
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&schema_issues)?),
            OutputFormat::Text => output::print_schema_issues(&schema_issues),
        }
        bail!("coverage schema validation failed");
    }

    let ir = load_core_ir(design_ir)?;
    let coverage_value: Value = serde_json::from_slice(&fs::read(coverage)?)?;
    let issues = design::coverage_design_issues(&ir, &coverage_value)?;
    let report = serde_json::json!({
        "status": if issues.is_empty() { "passed" } else { "failed" },
        "coverage": coverage.display().to_string(),
        "design_ir": design_ir.display().to_string(),
        "issues": issues
    });

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => output::print_coverage_check_text(&report),
    }

    if report.get("status").and_then(Value::as_str) == Some("passed") {
        Ok(())
    } else {
        bail!("coverage check failed")
    }
}
