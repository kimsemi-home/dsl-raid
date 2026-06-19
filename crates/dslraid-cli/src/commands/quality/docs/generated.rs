mod catalog;
mod cli;
mod hint;
mod index;
mod runscope;
mod script_bundle;
mod script_check;

use anyhow::Result;
use std::path::Path;

pub(super) fn check(input: &Path) -> Result<()> {
    runscope::check(input)?;
    cli::check(input)?;
    catalog::check(input)?;
    script_bundle::check()?;
    super::markers::check()?;
    index::check()
}
