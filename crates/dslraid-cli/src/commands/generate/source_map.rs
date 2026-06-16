use anyhow::Result;
use dslraid_codegen::generate_source_map;
use dslraid_core::CoreIr;
use std::path::Path;

pub(super) fn generate(input: &Path, ir: &CoreIr, out: &Path) -> Result<()> {
    let map = generate_source_map(ir, &input.display().to_string());
    let mut bytes = serde_json::to_vec_pretty(&map)?;
    bytes.push(b'\n');
    crate::write_bytes(out, &bytes)
}
