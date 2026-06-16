mod artifact;
mod fsm;
mod project;
mod table;

use dslraid_core::CoreIr;

pub fn generate_markdown_doc(ir: &CoreIr) -> String {
    let mut out = String::new();
    project::write(&mut out, ir);
    for fsm in &ir.fsms {
        fsm::write(&mut out, fsm);
    }
    artifact::write(&mut out, ir);
    out
}
