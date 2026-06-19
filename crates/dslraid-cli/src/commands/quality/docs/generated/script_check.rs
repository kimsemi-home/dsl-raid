use anyhow::{bail, Context, Result};
use std::process::Command;

pub(super) fn check(script: &str) -> Result<()> {
    let status = Command::new("bash")
        .arg(script)
        .arg("check")
        .status()
        .with_context(|| format!("run {script}"))?;
    if status.success() {
        Ok(())
    } else {
        bail!("{script} check failed")
    }
}
