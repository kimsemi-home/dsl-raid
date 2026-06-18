use anyhow::{anyhow, Result};
use dslraid_core::Fsm;

pub(super) fn initial_tuple(fsms: &[&Fsm]) -> Result<Vec<String>> {
    fsms.iter().map(initial_state_id).collect()
}

fn initial_state_id(fsm: &&Fsm) -> Result<String> {
    fsm.states
        .iter()
        .find(|state| state.initial)
        .or_else(|| fsm.states.first())
        .map(|state| state.id.clone())
        .ok_or_else(|| anyhow!("{} has no states", fsm.id))
}
