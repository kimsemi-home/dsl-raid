mod generated;
mod markers;
mod roadmap;
mod scripts;

use anyhow::Result;
use std::path::Path;

pub(super) fn check(input: &Path) -> Result<()> {
    generated::check(input)?;
    roadmap::check()
}
