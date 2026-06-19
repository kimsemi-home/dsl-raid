use anyhow::{Context, Result};
use std::path::Path;

const GOLDEN: &str = "docs/generated/cli-reference.md";

pub(super) fn check(input: &Path) -> Result<()> {
    crate::commands::outputs::doc(crate::DocArgs {
        command: crate::DocCommand::Cli {
            command: crate::CliDocCommand::Check {
                golden: Path::new(GOLDEN).to_path_buf(),
            },
        },
    })
    .with_context(|| super::hint::message(input))
}
