mod artifacts;
mod contracts;
mod rust_compile;
mod smoke;

use anyhow::Result;
use dslraid_core::CoreIr;
use std::path::Path;

pub(super) fn check(input: &Path, ir: &CoreIr) -> Result<()> {
    contracts::check()?;
    smoke::check(ir, input)?;
    rust_compile::check(ir)?;
    artifacts::check(ir, input)
}
