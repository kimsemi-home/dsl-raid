use super::target::codegen_target;
use anyhow::Result;
use dslraid_codegen::generate_code;
use dslraid_core::load_core_ir;
use std::fs;
use std::path::Path;

pub(crate) fn run(input: &Path, target: crate::CliCodegenTarget, out: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let target = codegen_target(target);
    let generated = generate_code(&ir, target)?;
    match out {
        Some(path) if path.extension().is_none() || path.is_dir() => {
            fs::create_dir_all(path)?;
            let file = path.join(format!("dslraid_generated.{}", target.extension()));
            crate::write_bytes(&file, generated.as_bytes())?;
            println!("generated {}", file.display());
            Ok(())
        }
        Some(path) => crate::write_bytes(path, generated.as_bytes()),
        None => {
            print!("{generated}");
            Ok(())
        }
    }
}
