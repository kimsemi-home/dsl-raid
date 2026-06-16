use std::collections::BTreeSet;

use dslraid_core::{CoreIr, Fsm};

pub(super) struct RefSets {
    pub(super) states: BTreeSet<String>,
    pub(super) events: BTreeSet<String>,
    pub(super) guards: BTreeSet<String>,
    pub(super) actions: BTreeSet<String>,
    pub(super) subjects: BTreeSet<String>,
}

impl RefSets {
    pub(super) fn new(fsm: &Fsm, ir: &CoreIr) -> Self {
        Self {
            states: fsm.states.iter().map(|state| state.id.clone()).collect(),
            events: fsm.events.iter().map(|event| event.id.clone()).collect(),
            guards: fsm.guards.iter().map(|guard| guard.id.clone()).collect(),
            actions: fsm.actions.iter().map(|action| action.id.clone()).collect(),
            subjects: ir.semantic_subjects(),
        }
    }
}
