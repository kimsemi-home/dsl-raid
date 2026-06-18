use anyhow::{anyhow, Result};
use dslraid_core::{Composition, CoreIr, Fsm};

pub(super) fn input_fsms<'a>(ir: &'a CoreIr, composition: &Composition) -> Result<Vec<&'a Fsm>> {
    composition
        .inputs
        .iter()
        .map(|id| {
            ir.find_fsm(id)
                .ok_or_else(|| anyhow!("unknown FSM input {id}"))
        })
        .collect()
}
