use anyhow::Result;
use std::fs;
use std::path::Path;

pub(super) fn check(input: &Path) -> Result<()> {
    check_trace(input)?;
    check_coverage(input)?;
    check_trace_import(input)?;
    check_trace_catalog()
}

fn check_trace(input: &Path) -> Result<()> {
    crate::commands::trace::check(
        Path::new("examples/runscope/run-001.trace.json"),
        input,
        crate::OutputFormat::Text,
    )
}

fn check_coverage(input: &Path) -> Result<()> {
    let coverage_path =
        std::env::temp_dir().join(format!("dslraid-coverage-{}.json", std::process::id()));
    crate::commands::coverage::build(
        Path::new("examples/runscope/run-001.trace.json"),
        input,
        Some(&coverage_path),
    )?;
    crate::schema_validate(
        Path::new("schemas/dslraid-coverage.schema.json"),
        &coverage_path,
    )?;
    crate::commands::coverage::check(&coverage_path, input, crate::OutputFormat::Text)?;
    fs::remove_file(&coverage_path).ok();
    Ok(())
}

fn check_trace_import(input: &Path) -> Result<()> {
    let imported_trace = std::env::temp_dir().join(format!(
        "dslraid-imported-trace-{}.json",
        std::process::id()
    ));
    crate::commands::trace::import(
        Path::new("examples/runscope/run-002.trace.jsonl"),
        Some(input),
        Some("run-002"),
        Some(&imported_trace),
    )?;
    crate::schema_validate(
        Path::new("schemas/dslraid-trace.schema.json"),
        &imported_trace,
    )?;
    fs::remove_file(&imported_trace).ok();
    Ok(())
}

fn check_trace_catalog() -> Result<()> {
    let status = std::process::Command::new("bash")
        .arg("scripts/tracegen.sh")
        .arg("check")
        .status()?;
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("scripts/tracegen.sh check failed")
    }
}
