mod artifacts;
mod smoke;

use anyhow::Result;
use dslraid_core::CoreIr;
use std::path::Path;

pub(super) fn check(input: &Path, ir: &CoreIr) -> Result<()> {
    smoke::check(ir, input)?;
    artifacts::check(ir, input)
}
