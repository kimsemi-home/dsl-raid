use crate::{fsm_local_name, state_subject, transition_subject, Fsm};

impl Fsm {
    pub fn local_name(&self) -> &str {
        fsm_local_name(&self.id)
    }

    pub fn state_subject(&self, state: &str) -> String {
        state_subject(&self.id, state)
    }

    pub fn transition_subject(&self, transition: &str) -> String {
        transition_subject(&self.id, transition)
    }
}
