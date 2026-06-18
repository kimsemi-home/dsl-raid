use anyhow::Result;
use dslraid_core::CoreIr;

use super::family::write_family;
use super::header::{canonical_header, go_header};
use crate::target::CodegenTarget;

pub fn generate_code(ir: &CoreIr, target: CodegenTarget) -> Result<String> {
    let mut out = String::new();
    match target {
        CodegenTarget::Rust => {
            write_family(&mut out, ir, super::rust::write_fsm, canonical_header())?
        }
        CodegenTarget::Go => write_family(&mut out, ir, super::go::write_fsm, go_header())?,
        CodegenTarget::TypeScript => write_family(
            &mut out,
            ir,
            super::typescript::write_fsm,
            canonical_header(),
        )?,
        CodegenTarget::Mermaid => write_family(&mut out, ir, super::mermaid::write_fsm, "")?,
        CodegenTarget::Dot => write_family(&mut out, ir, super::dot::write_fsm, "")?,
    }
    Ok(out)
}
