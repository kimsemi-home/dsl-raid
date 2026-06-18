mod dot;
mod go;
mod mermaid;
mod rust;
mod typescript;

#[cfg(test)]
mod tests;

use anyhow::Result;
use dslraid_core::CoreIr;

use crate::target::CodegenTarget;

pub fn generate_code(ir: &CoreIr, target: CodegenTarget) -> Result<String> {
    let mut out = String::new();
    match target {
        CodegenTarget::Rust => write_family(&mut out, ir, rust::write_fsm, canonical_header())?,
        CodegenTarget::Go => write_family(&mut out, ir, go::write_fsm, go_header())?,
        CodegenTarget::TypeScript => {
            write_family(&mut out, ir, typescript::write_fsm, canonical_header())?
        }
        CodegenTarget::Mermaid => write_family(&mut out, ir, mermaid::write_fsm, "")?,
        CodegenTarget::Dot => write_family(&mut out, ir, dot::write_fsm, "")?,
    }
    Ok(out)
}

fn write_family(
    out: &mut String,
    ir: &CoreIr,
    writer: fn(&mut String, &dslraid_core::Fsm) -> Result<()>,
    header: &str,
) -> Result<()> {
    out.push_str(header);
    for (index, fsm) in ir.fsms.iter().enumerate() {
        writer(out, fsm)?;
        if index + 1 < ir.fsms.len() && !out.ends_with("\n\n") {
            out.push('\n');
        }
    }
    Ok(())
}

fn canonical_header() -> &'static str {
    "// Generated from DSLRaid Canonical IR by dslraid-codegen. Do not edit by hand.\n\n"
}

fn go_header() -> &'static str {
    "\
// Generated from DSLRaid Canonical IR by dslraid-codegen. Do not edit by hand.

package generated

func dslraidNext[S comparable](state S, event string, transitions map[S]map[string]S) (S, bool) {
\tnext, ok := transitions[state][event]
\tif !ok {
\t\treturn state, false
\t}
\treturn next, true
}

"
}
