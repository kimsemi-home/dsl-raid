use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

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
    .and_then(|_| check_assertion_catalog())
    .and_then(|_| check_generated_docs_index())
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

fn check_assertion_catalog() -> Result<()> {
    check_script("scripts/assertiongen.sh")
}

fn check_generated_docs_index() -> Result<()> {
    check_script("scripts/gendocindex.sh")
}

fn check_script(script: &str) -> Result<()> {
    let status = Command::new("bash")
        .arg(script)
        .arg("check")
        .status()
        .with_context(|| format!("run {script}"))?;
    if status.success() {
        Ok(())
    } else {
        bail!("{script} check failed")
    }
}

fn hint(input: &Path) -> String {
    format!(
        "refresh generated docs with `dslraid generate {} --cli-doc docs/generated/cli-reference.md`",
        input.display()
    )
}
