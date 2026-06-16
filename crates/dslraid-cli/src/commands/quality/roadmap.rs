use anyhow::{bail, Context, Result};
use std::process::Command;

pub(super) fn check() -> Result<()> {
    let status = Command::new("bash")
        .arg("scripts/roadmapgen.sh")
        .arg("check")
        .status()
        .context("run scripts/roadmapgen.sh")?;
    if status.success() {
        Ok(())
    } else {
        bail!("scripts/roadmapgen.sh check failed")
    }
}
