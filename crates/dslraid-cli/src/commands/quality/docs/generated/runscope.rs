use anyhow::{Context, Result};
use std::path::Path;

const GOLDEN: &str = "examples/runscope/runscope.generated.md";

pub(super) fn check(input: &Path) -> Result<()> {
    crate::commands::outputs::doc(crate::DocArgs {
        command: crate::DocCommand::Check {
            input: input.to_path_buf(),
            golden: Path::new(GOLDEN).to_path_buf(),
        },
    })
    .with_context(|| super::hint::message(input))
}
