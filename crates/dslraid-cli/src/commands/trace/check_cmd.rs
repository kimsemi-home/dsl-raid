use super::{design, output};
use crate::OutputFormat;
use anyhow::{bail, Result};
use dslraid_core::{load_core_ir, validate_json_schema};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub(crate) fn check(trace: &Path, design_ir: &Path, format: OutputFormat) -> Result<()> {
    let schema_issues =
        validate_json_schema(Path::new("schemas/dslraid-trace.schema.json"), trace)?;
    if !schema_issues.is_empty() {
        match format {
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&schema_issues)?),
            OutputFormat::Text => output::print_schema_issues(&schema_issues),
        }
        bail!("trace schema validation failed");
    }
    if matches!(format, OutputFormat::Text) {
        println!("schema ok: {}", trace.display());
    }

    let ir = load_core_ir(design_ir)?;
    let trace_value: Value = serde_json::from_slice(&fs::read(trace)?)?;
    let issues = design::trace_design_issues(&ir, &trace_value)?;
    let report = serde_json::json!({
        "status": if issues.is_empty() { "passed" } else { "failed" },
        "trace": trace.display().to_string(),
        "design_ir": design_ir.display().to_string(),
        "issues": issues
    });

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => output::print_trace_check_text(&report),
    }

    if report.get("status").and_then(Value::as_str) == Some("passed") {
        Ok(())
    } else {
        bail!("trace check failed")
    }
}
