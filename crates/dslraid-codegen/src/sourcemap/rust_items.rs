mod state;
mod transition;

use dslraid_core::{Artifact, Fsm};

use super::generated::Index;

pub(super) fn add_fsm(
    artifact: &Artifact,
    index: &mut Index,
    fsm: &Fsm,
    start: usize,
    end: usize,
    lines: &[&str],
) {
    super::push::generated(index, artifact, fsm.id.clone(), start, end);
    for state in &fsm.states {
        state::add_state(artifact, index, fsm, state, start, end, lines);
    }
    for transition in &fsm.transitions {
        transition::add_transition(artifact, index, fsm, transition, start, end, lines);
    }
}
