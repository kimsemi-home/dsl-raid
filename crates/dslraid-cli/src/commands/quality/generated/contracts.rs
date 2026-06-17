use anyhow::{bail, Result};
use dslraid_codegen::CodegenTarget;

pub(super) fn check() -> Result<()> {
    for target in CodegenTarget::ALL {
        check_contract(target)?;
    }
    Ok(())
}

fn check_contract(target: CodegenTarget) -> Result<()> {
    let contract = target.contract();
    if contract.input != "Canonical IR" {
        bail!("backend {target:?} must consume Canonical IR");
    }
    if !contract.lossy && !contract.role.contains("source") {
        bail!("lossless backend {target:?} must declare a source role");
    }
    check_lisp_ssot_notice(target, contract.contract)
}

fn check_lisp_ssot_notice(target: CodegenTarget, contract: &str) -> Result<()> {
    if !contract.contains("Lisp forms stay SSOT") {
        return Ok(());
    }
    if matches!(target, CodegenTarget::Rust) {
        return Ok(());
    }
    bail!("only Rust should carry the runtime/codegen SSOT reminder")
}
