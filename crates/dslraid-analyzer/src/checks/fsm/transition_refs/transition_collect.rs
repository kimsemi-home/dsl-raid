use dslraid_core::{transition_subject, Fsm, Transition};

use super::failures::RefFailures;
use super::sets::RefSets;
use super::{event_collect, guard_action_collect, requires_collect, state_collect};

pub(super) fn collect(
    fsm: &Fsm,
    transition: &Transition,
    refs: &RefSets,
    failures: &mut RefFailures,
) {
    let subject = transition_subject(&fsm.id, &transition.id);
    state_collect::collect(refs, failures, &subject, transition);
    event_collect::collect(refs, failures, &subject, transition);
    guard_action_collect::collect(refs, failures, &subject, transition);
    requires_collect::collect(refs, failures, &subject, transition);
}
