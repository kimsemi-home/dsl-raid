use anyhow::Result;
use std::path::Path;

const TRACE: &str = "examples/runscope/run-001.trace.json";

pub(super) fn check(input: &Path) -> Result<()> {
    crate::commands::trace::check(Path::new(TRACE), input, crate::OutputFormat::Text)
}
