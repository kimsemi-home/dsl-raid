use anyhow::{bail, Result};
use dslraid_codegen::generate_markdown_doc;
use dslraid_core::{Artifact, CoreIr};
use std::path::Path;

pub(super) fn generate_ir(input: &Path, ir: &CoreIr, artifact: &Artifact) -> Result<()> {
    if Path::new(&artifact.path)
        .extension()
        .and_then(|value| value.to_str())
        != Some("md")
    {
        bail!("unsupported doc artifact path: {}", artifact.path);
    }
    let path = crate::commands::artifact::resolve_artifact_path(input, &artifact.path);
    let markdown = generate_markdown_doc(ir);
    crate::write_bytes(&path, markdown.as_bytes())
}

pub(super) fn generate_cli(path: &Path) -> Result<()> {
    crate::commands::outputs::doc(crate::DocArgs {
        command: crate::DocCommand::Cli {
            command: crate::CliDocCommand::Generate {
                out: Some(path.to_path_buf()),
            },
        },
    })
}
