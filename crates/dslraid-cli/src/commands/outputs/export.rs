use super::target::export_target;
use anyhow::Result;
use dslraid_codegen::generate_code;
use dslraid_core::load_core_ir;
use std::path::Path;

pub(crate) fn run(input: &Path, target: crate::CliExportTarget, out: Option<&Path>) -> Result<()> {
    match target {
        crate::CliExportTarget::Json => export_json(input, out),
        crate::CliExportTarget::Svg => {
            super::render::run(input, None, crate::RenderFormat::Svg, out)
        }
        crate::CliExportTarget::Mermaid | crate::CliExportTarget::Dot => {
            let ir = load_core_ir(input)?;
            let generated = generate_code(&ir, export_target(target))?;
            crate::write_or_stdout(out, generated.as_bytes())
        }
    }
}

fn export_json(input: &Path, out: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let bytes = serde_json::to_vec_pretty(&ir)?;
    crate::write_or_stdout(out, &bytes)
}
