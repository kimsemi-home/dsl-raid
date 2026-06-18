use dslraid_core::{Fsm, Transition};

pub(super) fn outgoing<'a>(fsm: &'a Fsm, state: &str) -> Vec<&'a Transition> {
    fsm.transitions
        .iter()
        .filter(|transition| transition.from == state)
        .collect()
}
