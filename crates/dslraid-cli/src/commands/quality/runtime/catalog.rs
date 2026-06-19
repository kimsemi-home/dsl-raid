use anyhow::{bail, Result};
use std::process::Command;

pub(super) fn check() -> Result<()> {
    let status = Command::new("bash")
        .arg("scripts/tracegen.sh")
        .arg("check")
        .status()?;
    if status.success() {
        Ok(())
    } else {
        bail!("scripts/tracegen.sh check failed")
    }
}
