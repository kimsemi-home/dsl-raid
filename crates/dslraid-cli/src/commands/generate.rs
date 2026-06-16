mod artifact;
mod code;
mod docs;
mod target;
mod validation;

#[cfg(test)]
mod tests;

use anyhow::Result;
use dslraid_core::load_core_ir;

pub(crate) fn run(args: crate::GenerateArgs) -> Result<()> {
    let ir = load_core_ir(&args.input)?;
    for artifact in &ir.artifacts {
        artifact::generate(&args.input, &ir, artifact)?;
    }
    if let Some(path) = args.cli_doc.as_deref() {
        docs::generate_cli(path)?;
    }
    if let Some(path) = args.validation_report.as_deref() {
        validation::generate(&args.input, &ir, path)?;
    }
    if !args.skip_lock {
        crate::commands::artifact::update_lock(&args.input, None)?;
    }
    Ok(())
}
