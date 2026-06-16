use anyhow::{Context, Result};
use dslraid_codegen::generate_source_map;
use dslraid_core::load_core_ir;
use std::fs;
use std::path::Path;

pub(super) fn run(input: &Path, out: &Path, trace: Option<&Path>) -> Result<()> {
    fs::create_dir_all(out)?;
    crate::commands::outputs::render(input, None, crate::RenderFormat::Svg, Some(out))?;
    write_source_map(input, out)?;
    if let Some(trace) = trace {
        copy_trace(trace, out)?;
        write_coverage(input, trace, out)?;
    }
    println!("packaged demo {}", out.display());
    Ok(())
}

fn write_source_map(input: &Path, out: &Path) -> Result<()> {
    let ir = load_core_ir(input)?;
    let map = generate_source_map(&ir, &input.display().to_string());
    let mut bytes = serde_json::to_vec_pretty(&map)?;
    bytes.push(b'\n');
    let path = out.join(super::names::source_map(input)?);
    crate::write_bytes(&path, &bytes)
}

fn copy_trace(trace: &Path, out: &Path) -> Result<()> {
    let name = trace
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("trace has no file name"))?;
    fs::copy(trace, out.join(name)).with_context(|| format!("copy trace {}", trace.display()))?;
    Ok(())
}

fn write_coverage(input: &Path, trace: &Path, out: &Path) -> Result<()> {
    let coverage = out.join(super::names::coverage(trace)?);
    crate::commands::coverage::build(trace, input, Some(&coverage))
}
