use dslraid_core::{CoreIr, Fsm};

use super::failures::RefFailures;
use super::sets::RefSets;
use super::transition_collect;

pub(super) fn collect(fsm: &Fsm, ir: &CoreIr) -> RefFailures {
    let refs = RefSets::new(fsm, ir);
    let mut failures = RefFailures::new();
    for transition in &fsm.transitions {
        transition_collect::collect(fsm, transition, &refs, &mut failures);
    }
    failures
}
