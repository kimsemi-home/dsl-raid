use anyhow::{bail, Context, Result};
use dslraid_codegen::generate_markdown_doc;
use dslraid_core::load_core_ir;
use std::fs;
use std::path::Path;

pub(crate) fn run(args: crate::DocArgs) -> Result<()> {
    match args.command {
        crate::DocCommand::Generate { input, out } => generate(&input, out.as_deref()),
        crate::DocCommand::Check { input, golden } => check(&input, &golden),
    }
}

pub(crate) fn generate(input: &Path, out: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let markdown = generate_markdown_doc(&ir);
    crate::write_or_stdout(out, markdown.as_bytes())
}

pub(crate) fn check(input: &Path, golden: &Path) -> Result<()> {
    let ir = load_core_ir(input)?;
    let actual = generate_markdown_doc(&ir);
    let expected = fs::read_to_string(golden)
        .with_context(|| format!("read generated doc {}", golden.display()))?;
    if actual != expected {
        bail!(
            "generated doc is stale: run `dslraid doc generate {} --out {}`",
            input.display(),
            golden.display()
        );
    }
    Ok(())
}
