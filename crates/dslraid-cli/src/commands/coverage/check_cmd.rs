use super::design::coverage_design_issues;
use super::output::{print_coverage_check_text, print_schema_issues};
use crate::OutputFormat;
use anyhow::{bail, Result};
use dslraid_core::{load_core_ir, validate_json_schema};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub(crate) fn check(coverage: &Path, design_ir: &Path, format: OutputFormat) -> Result<()> {
    let schema_issues =
        validate_json_schema(Path::new("schemas/dslraid-coverage.schema.json"), coverage)?;
    if !schema_issues.is_empty() {
        match format {
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&schema_issues)?),
            OutputFormat::Text => print_schema_issues(&schema_issues),
        }
        bail!("coverage schema validation failed");
    }

    let ir = load_core_ir(design_ir)?;
    let coverage_value: Value = serde_json::from_slice(&fs::read(coverage)?)?;
    let issues = coverage_design_issues(&ir, &coverage_value)?;
    let status = if issues.is_empty() {
        "passed"
    } else {
        "failed"
    };
    let report = serde_json::json!({
        "status": status,
        "coverage": coverage.display().to_string(),
        "design_ir": design_ir.display().to_string(),
        "issues": issues
    });

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => print_coverage_check_text(&report),
    }

    if status == "passed" {
        Ok(())
    } else {
        bail!("coverage check failed")
    }
}
