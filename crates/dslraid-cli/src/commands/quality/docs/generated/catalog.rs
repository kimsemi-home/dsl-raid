use anyhow::Result;
use std::path::Path;

const GOLDEN: &str = "docs/generated/fsm-catalog.md";

pub(super) fn check(input: &Path) -> Result<()> {
    crate::commands::outputs::doc(crate::DocArgs {
        command: crate::DocCommand::FsmCatalog {
            command: crate::FsmCatalogDocCommand::Check {
                input: input.to_path_buf(),
                golden: Path::new(GOLDEN).to_path_buf(),
            },
        },
    })?;
    println!("fsm generated doc ok");
    Ok(())
}
