use anyhow::Result;
use dslraid_core::load_core_ir;
use std::path::Path;

pub(crate) fn run(input: &Path, out: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let bytes = serde_json::to_vec_pretty(&ir)?;
    crate::write_or_stdout(out, &bytes)
}
