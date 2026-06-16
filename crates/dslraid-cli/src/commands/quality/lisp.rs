use anyhow::{bail, Context, Result};
use dslraid_core::{load_core_ir, CORE_SCHEMA_PATH};
use std::path::Path;
use std::process::Command;

pub(super) fn check() -> Result<()> {
    run_script("scripts/lisp-docgen.sh")?;
    run_lisp_irgen()?;
    run_script("scripts/lisp-rustgen.sh")?;
    validate_lisp_ir(Path::new("examples/runscope/runscope.lisp.raid.json"))
}

fn run_script(script: &str) -> Result<()> {
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

fn run_lisp_irgen() -> Result<()> {
    let status = Command::new("bash")
        .arg("scripts/lisp-irgen.sh")
        .arg("check")
        .env("DSLRAID_SKIP_RUST_VALIDATE", "1")
        .status()
        .context("run scripts/lisp-irgen.sh")?;
    if status.success() {
        Ok(())
    } else {
        bail!("scripts/lisp-irgen.sh check failed")
    }
}

fn validate_lisp_ir(path: &Path) -> Result<()> {
    crate::schema_validate(Path::new(CORE_SCHEMA_PATH), path)?;
    let ir = load_core_ir(path)?;
    let deny = vec!["warning".to_string()];
    let report = crate::validation_report(&ir, path, "quality", deny.clone())?;
    if report.is_success(&deny) {
        Ok(())
    } else {
        crate::print_report_text(&report);
        bail!("generated Lisp IR validation failed")
    }
}
