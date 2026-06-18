use dslraid_core::{transition_subject, Fsm};

pub(super) fn transitions(fsm: &Fsm, transitions: &[String]) -> Vec<String> {
    transitions
        .iter()
        .map(|transition| transition_subject(&fsm.id, transition))
        .collect()
}
