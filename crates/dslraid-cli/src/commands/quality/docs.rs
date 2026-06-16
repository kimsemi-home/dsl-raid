use anyhow::{Context, Result};
use std::path::Path;

pub(super) fn check(input: &Path) -> Result<()> {
    crate::commands::outputs::doc(crate::DocArgs {
        command: crate::DocCommand::Check {
            input: input.to_path_buf(),
            golden: Path::new("examples/runscope/runscope.generated.md").to_path_buf(),
        },
    })
    .with_context(|| hint(input))?;
    crate::commands::outputs::doc(crate::DocArgs {
        command: crate::DocCommand::Cli {
            command: crate::CliDocCommand::Check {
                golden: Path::new("docs/generated/cli-reference.md").to_path_buf(),
            },
        },
    })
    .with_context(|| hint(input))
    .and_then(|_| check_fsm_catalog(input))
}

fn check_fsm_catalog(input: &Path) -> Result<()> {
    crate::commands::outputs::doc(crate::DocArgs {
        command: crate::DocCommand::FsmCatalog {
            command: crate::FsmCatalogDocCommand::Check {
                input: input.to_path_buf(),
                golden: Path::new("docs/generated/fsm-catalog.md").to_path_buf(),
            },
        },
    })?;
    println!("fsm generated doc ok");
    Ok(())
}

fn hint(input: &Path) -> String {
    format!(
        "refresh generated docs with `dslraid generate {} --cli-doc docs/generated/cli-reference.md`",
        input.display()
    )
}
