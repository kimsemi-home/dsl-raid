use super::{output, result, RunOptions};
use crate::{write_or_stdout, OutputFormat};
use anyhow::Result;
use dslraid_core::load_core_ir;

pub(crate) fn run(options: RunOptions<'_>) -> Result<()> {
    let ir = load_core_ir(options.input)?;
    let result = result::result(
        &ir,
        options.composition,
        options.materialize,
        options.limit,
        options.focus,
        options.depth,
    )?;
    let bytes = match options.format {
        OutputFormat::Json => serde_json::to_vec_pretty(&result)?,
        OutputFormat::Text => output::text(&result, options.materialize)?.into_bytes(),
    };
    write_or_stdout(options.out, &bytes)
}
