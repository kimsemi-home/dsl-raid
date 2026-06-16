use anyhow::{bail, Context, Result};
use dslraid_codegen::{generate_code, CodegenTarget};
use dslraid_core::{Artifact, CoreIr};
use std::fs;
use std::path::Path;

pub(super) fn check(ir: &CoreIr, input: &Path) -> Result<()> {
    for artifact in &ir.artifacts {
        let Some(target) = target_for(artifact) else {
            continue;
        };
        check_one(ir, input, artifact, target)?;
    }
    Ok(())
}

fn check_one(ir: &CoreIr, input: &Path, artifact: &Artifact, target: CodegenTarget) -> Result<()> {
    let expected = generate_code(ir, target)?;
    let path = crate::commands::artifact::resolve_artifact_path(input, &artifact.path);
    let actual = fs::read_to_string(&path)
        .with_context(|| format!("read generated artifact {}", artifact.path))?;
    if actual != expected {
        bail!(
            "generated artifact {} is stale: run `dslraid generate {}`",
            artifact.id,
            input.display(),
        );
    }
    Ok(())
}

fn target_for(artifact: &Artifact) -> Option<CodegenTarget> {
    if artifact.kind != "generated" {
        return None;
    }
    match Path::new(&artifact.path).extension()?.to_str()? {
        "rs" => Some(CodegenTarget::Rust),
        "go" => Some(CodegenTarget::Go),
        "ts" => Some(CodegenTarget::TypeScript),
        "mmd" => Some(CodegenTarget::Mermaid),
        "dot" => Some(CodegenTarget::Dot),
        _ => None,
    }
}
