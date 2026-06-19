use anyhow::{anyhow, Result};
use dslraid_core::{Fsm, Transition};

use crate::view::ViewNode;

pub(super) fn node_for<'a>(
    fsm: &Fsm,
    transition: &Transition,
    nodes: &'a [ViewNode],
    state_id: &str,
) -> Result<&'a ViewNode> {
    let index = fsm
        .states
        .iter()
        .position(|state| state.id == state_id)
        .ok_or_else(|| unknown_state(transition, state_id))?;
    Ok(&nodes[index])
}

fn unknown_state(transition: &Transition, state_id: &str) -> anyhow::Error {
    anyhow!(
        "transition {} has unknown state {}",
        transition.id,
        state_id
    )
}
