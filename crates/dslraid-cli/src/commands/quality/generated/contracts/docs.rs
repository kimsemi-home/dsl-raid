use anyhow::{bail, Context, Result};
use dslraid_codegen::CodegenTarget;
use std::fs;

const BACKEND_DOC: &str = "docs/generated/backend-targets.md";

pub(super) fn check() -> Result<()> {
    let content =
        fs::read_to_string(BACKEND_DOC).with_context(|| format!("failed to read {BACKEND_DOC}"))?;
    for target in CodegenTarget::ALL {
        check_target(&content, target)?;
    }
    Ok(())
}

fn check_target(content: &str, target: CodegenTarget) -> Result<()> {
    let row = expected_row(target);
    if content.lines().any(|line| line == row) {
        return Ok(());
    }
    bail!("backend target doc is missing codegen contract row: {row}")
}

fn expected_row(target: CodegenTarget) -> String {
    let contract = target.contract();
    format!(
        "| {} | {} | {} | {} | {} |",
        target.id(),
        contract.role,
        contract.input,
        contract.lossy_label(),
        contract.contract
    )
}
