use super::output::print_items;
use super::select::values;
use crate::OutputFormat;
use anyhow::Result;
use dslraid_core::load_core_ir;
use std::path::Path;

pub(crate) fn run(input: &Path, expression: &str, format: OutputFormat) -> Result<()> {
    let ir = load_core_ir(input)?;
    let items = values(&ir, expression)?;
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&items)?),
        OutputFormat::Text => print_items(items),
    }
    Ok(())
}
