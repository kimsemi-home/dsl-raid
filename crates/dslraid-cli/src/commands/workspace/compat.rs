use anyhow::Result;
use dslraid_core::load_core_ir;
use std::path::Path;

pub(crate) fn run(input: &Path) -> Result<()> {
    let ir = load_core_ir(input)?;
    println!(
        "compat ok: ir_version={} project={}",
        ir.ir_version, ir.project.id
    );
    Ok(())
}
