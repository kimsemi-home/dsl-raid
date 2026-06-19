mod catalog;
mod coverage;
mod temp_file;
mod trace;
mod trace_import;

use anyhow::Result;
use std::path::Path;

pub(super) fn check(input: &Path) -> Result<()> {
    trace::check(input)?;
    coverage::check(input)?;
    trace_import::check(input)?;
    catalog::check()
}
