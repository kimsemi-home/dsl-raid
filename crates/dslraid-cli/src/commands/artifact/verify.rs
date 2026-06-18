mod artifact_row;
mod checks;
mod content_hash;
mod derivation;
mod issue;
mod output;
mod output_issue;
mod output_summary;
mod record;
mod record_hash;
mod record_status;
mod report;

use anyhow::{bail, Result};
use serde_json::Value;
use std::path::Path;

pub(super) fn run(input: &Path, lock: Option<&Path>, format: crate::OutputFormat) -> Result<()> {
    let report = report::build(input, lock)?;
    match format {
        crate::OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        crate::OutputFormat::Text => output::print_text(&report),
    }
    ensure_passed(&report)
}

#[cfg(test)]
pub(super) fn build_report(input: &Path, lock: Option<&Path>) -> Result<Value> {
    report::build(input, lock)
}

fn ensure_passed(report: &Value) -> Result<()> {
    if report.get("status").and_then(Value::as_str) == Some("passed") {
        Ok(())
    } else {
        bail!("artifact verification failed")
    }
}
