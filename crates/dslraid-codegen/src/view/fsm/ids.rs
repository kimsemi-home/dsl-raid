use dslraid_core::{fsm_local_name, Fsm};

pub(crate) fn layout_state_id(fsm: &Fsm, state: &str) -> String {
    format!("layout:{}.state.{}", fsm_local_name(&fsm.id), state)
}

pub(crate) fn layout_transition_id(fsm: &Fsm, transition: &str) -> String {
    format!(
        "layout:{}.transition.{}",
        fsm_local_name(&fsm.id),
        transition
    )
}
