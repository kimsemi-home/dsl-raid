use anyhow::{bail, Result};
use dslraid_core::{Artifact, CoreIr};
use std::path::Path;

pub(super) fn generate(input: &Path, ir: &CoreIr, artifact: &Artifact) -> Result<()> {
    match artifact.kind.as_str() {
        "generated" => super::code::generate(input, ir, artifact),
        "doc" => super::docs::generate_ir(input, ir, artifact),
        "source" | "test" => Ok(()),
        other => bail!("unsupported artifact kind for generation: {other}"),
    }
}
