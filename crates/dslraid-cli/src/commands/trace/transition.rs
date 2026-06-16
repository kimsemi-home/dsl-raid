use dslraid_core::{state_subject, transition_subject, CoreIr};

pub(super) fn transition_endpoints(ir: &CoreIr, subject: &str) -> Option<(String, String)> {
    for fsm in &ir.fsms {
        for transition in &fsm.transitions {
            if transition_subject(&fsm.id, &transition.id) == subject {
                return Some((
                    state_subject(&fsm.id, &transition.from),
                    state_subject(&fsm.id, &transition.to),
                ));
            }
        }
    }
    None
}
