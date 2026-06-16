use anyhow::Result;
use std::path::Path;

pub(super) fn check(input: &Path) -> Result<()> {
    crate::commands::outputs::doc(crate::DocArgs {
        command: crate::DocCommand::Check {
            input: input.to_path_buf(),
            golden: Path::new("examples/runscope/runscope.generated.md").to_path_buf(),
        },
    })
}
