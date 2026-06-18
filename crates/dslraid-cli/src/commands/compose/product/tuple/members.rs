use dslraid_core::{state_subject, Fsm};

pub(in crate::commands::compose) fn tuple_members(fsms: &[&Fsm], tuple: &[String]) -> Vec<String> {
    fsms.iter()
        .zip(tuple.iter())
        .map(|(fsm, state)| state_subject(&fsm.id, state))
        .collect()
}
