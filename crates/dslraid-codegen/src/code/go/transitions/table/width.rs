use dslraid_core::Fsm;

use crate::names::go_type;

pub(super) fn row_width(fsm: &Fsm, type_name: &str) -> usize {
    fsm.states
        .iter()
        .filter(|state| !super::outgoing::outgoing(fsm, &state.id).is_empty())
        .map(|state| format!("{type_name}State{}", go_type(&state.id)).len())
        .max()
        .unwrap_or_default()
}
