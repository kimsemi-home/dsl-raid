use anyhow::{anyhow, Result};
use dslraid_core::{Composition, CoreIr, Fsm};

pub(super) fn resolve_input_fsms<'a>(
    ir: &'a CoreIr,
    composition: &Composition,
) -> Result<Vec<&'a Fsm>> {
    composition
        .inputs
        .iter()
        .map(|id| {
            ir.find_fsm(id).ok_or_else(|| {
                anyhow!(
                    "composition {} references unknown FSM {}",
                    composition.id,
                    id
                )
            })
        })
        .collect()
}

pub(super) fn state_space(fsms: &[&Fsm]) -> usize {
    fsms.iter().map(|fsm| fsm.states.len().max(1)).product()
}
