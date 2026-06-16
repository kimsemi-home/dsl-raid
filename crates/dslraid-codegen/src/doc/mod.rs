mod artifact;
mod catalog;
mod fsm;
mod project;
mod table;

use dslraid_core::CoreIr;

#[cfg(test)]
mod tests;

pub fn generate_markdown_doc(ir: &CoreIr) -> String {
    let mut out = String::new();
    project::write(&mut out, ir);
    for fsm in &ir.fsms {
        fsm::write(&mut out, fsm);
    }
    artifact::write(&mut out, ir);
    out
}

pub fn generate_fsm_catalog_doc(ir: &CoreIr) -> String {
    catalog::write(ir)
}
