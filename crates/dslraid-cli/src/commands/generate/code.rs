use anyhow::Result;
use dslraid_codegen::generate_code;
use dslraid_core::{Artifact, CoreIr};
use std::path::Path;

pub(super) fn generate(input: &Path, ir: &CoreIr, artifact: &Artifact) -> Result<()> {
    let target = super::target::codegen_target(&artifact.path)?;
    let path = crate::commands::artifact::resolve_artifact_path(input, &artifact.path);
    let generated = generate_code(ir, target)?;
    crate::write_bytes(&path, generated.as_bytes())
}
