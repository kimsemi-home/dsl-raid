use dslraid_core::{Artifact, CoreIr, Fsm};

use crate::names::rust_type;

use super::generated::Index;

pub(super) fn add(ir: &CoreIr, artifact: &Artifact, index: &mut Index) {
    let Ok(code) = crate::code::generate_code(ir, crate::CodegenTarget::Rust) else {
        return;
    };
    let lines = code.lines().collect::<Vec<_>>();
    let starts = starts(ir, &lines);
    for (position, (fsm, start)) in starts.iter().enumerate() {
        let end = super::lines::span_end(&starts, position, lines.len());
        super::rust_items::add_fsm(artifact, index, fsm, *start, end, &lines);
    }
}

fn starts<'a>(ir: &'a CoreIr, lines: &[&str]) -> Vec<(&'a Fsm, usize)> {
    ir.fsms
        .iter()
        .filter_map(|fsm| {
            let enum_line = super::lines::find(
                lines,
                1,
                lines.len(),
                &format!("pub enum {}State", rust_type(&fsm.name)),
            )?;
            Some((fsm, enum_line.saturating_sub(1).max(1)))
        })
        .collect()
}
