use crate::{write_or_stdout, DiffFormat};
use anyhow::Result;
use dslraid_core::load_core_ir;
use std::path::Path;

pub(crate) fn run(base: &Path, head: &Path, format: DiffFormat, out: Option<&Path>) -> Result<()> {
    let base_ir = load_core_ir(base)?;
    let head_ir = load_core_ir(head)?;
    let report = super::report::report(&base_ir, &head_ir, base, head)?;
    let bytes = match format {
        DiffFormat::Json => serde_json::to_vec_pretty(&report)?,
        DiffFormat::Markdown => super::render::markdown_report(&report).into_bytes(),
        DiffFormat::Text => super::render::text_report(&report).into_bytes(),
    };
    write_or_stdout(out, &bytes)
}
