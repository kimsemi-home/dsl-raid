use dslraid_core::{state_subject, Fsm};

use super::reachable;

pub(super) fn states(fsm: &Fsm) -> Vec<String> {
    let reachable = reachable::states(fsm);
    fsm.states
        .iter()
        .filter(|state| !reachable.contains(&state.id))
        .map(|state| state_subject(&fsm.id, &state.id))
        .collect()
}
