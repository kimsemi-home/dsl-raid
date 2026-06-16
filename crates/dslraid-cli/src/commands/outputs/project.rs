use anyhow::Result;
use dslraid_codegen::project_view;
use dslraid_core::load_core_ir;
use std::path::Path;

pub(crate) fn run(input: &Path, projection: Option<&str>, out: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let view = project_view(&ir, projection, input.display().to_string())?;
    let bytes = serde_json::to_vec_pretty(&view)?;
    crate::write_or_stdout(out, &bytes)
}
