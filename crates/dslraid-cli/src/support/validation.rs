use anyhow::Result;
use dslraid_analyzer::{validate_core_ir, ValidateOptions, ValidationReport};
use dslraid_core::{sha256_json, CoreIr};
use std::path::Path;

pub(crate) fn validation_report(
    ir: &CoreIr,
    input: &Path,
    mode: &str,
    deny: Vec<String>,
) -> Result<ValidationReport> {
    let hash = sha256_json(ir)?;
    Ok(validate_core_ir(
        ir,
        ValidateOptions {
            source_path: input.display().to_string(),
            ir_hash: Some(hash),
            mode: mode.to_string(),
            deny,
        },
    ))
}

pub(crate) fn print_report_text(report: &ValidationReport) {
    println!("validation {}", report.summary.status);
    println!(
        "assertions: passed={} failed={} warnings={} skipped={} n/a={}",
        report.summary.assertions.passed,
        report.summary.assertions.failed,
        report.summary.assertions.warnings,
        report.summary.assertions.skipped,
        report.summary.assertions.not_applicable
    );
    for assertion in &report.assertions {
        if assertion.status != "passed" && assertion.status != "not_applicable" {
            println!(
                "{} {} {}: {}",
                assertion.severity,
                assertion.code,
                assertion.id,
                assertion.message.clone().unwrap_or_default()
            );
            if let Some(suggestion) = &assertion.suggestion {
                println!("  suggestion: {suggestion}");
            }
        }
    }
}
