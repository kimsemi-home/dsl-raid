mod artifacts;
mod contracts;
mod smoke;

use anyhow::Result;
use dslraid_core::CoreIr;
use std::path::Path;

pub(super) fn check(input: &Path, ir: &CoreIr) -> Result<()> {
    contracts::check()?;
    smoke::check(ir, input)?;
    artifacts::check(ir, input)
}
