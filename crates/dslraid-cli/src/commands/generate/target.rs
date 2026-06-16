use anyhow::{bail, Result};
use dslraid_codegen::CodegenTarget;
use std::path::Path;

pub(super) fn codegen_target(path: &str) -> Result<CodegenTarget> {
    match Path::new(path).extension().and_then(|value| value.to_str()) {
        Some("rs") => Ok(CodegenTarget::Rust),
        Some("go") => Ok(CodegenTarget::Go),
        Some("ts") => Ok(CodegenTarget::TypeScript),
        Some("mmd") => Ok(CodegenTarget::Mermaid),
        Some("dot") => Ok(CodegenTarget::Dot),
        Some(_) => bail!("unsupported generated artifact path: {path}"),
        None => bail!("generated artifact path has no extension: {path}"),
    }
}
