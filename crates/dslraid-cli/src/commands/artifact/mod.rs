mod artifact_row;
mod checks;
mod content_hash;
mod derivation;
mod issue;
mod kind;
mod lock;
mod lock_update;
mod output;
mod path;
mod record;
mod record_hash;
mod record_status;
mod report;

#[cfg(test)]
mod tests;

use crate::OutputFormat;
use anyhow::{bail, Result};
use serde_json::Value;
use std::path::Path;

pub(crate) fn verify(input: &Path, lock: Option<&Path>, format: OutputFormat) -> Result<()> {
    let report = report::build(input, lock)?;
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => output::print_text(&report),
    }
    if report.get("status").and_then(Value::as_str) == Some("passed") {
        Ok(())
    } else {
        bail!("artifact verification failed")
    }
}

pub(crate) fn update_lock(input: &Path, out: Option<&Path>) -> Result<()> {
    lock_update::run(input, out)
}
