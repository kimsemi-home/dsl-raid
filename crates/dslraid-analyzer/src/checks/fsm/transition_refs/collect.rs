use dslraid_core::{transition_subject, CoreIr, Fsm, Transition};

use super::failures::RefFailures;
use super::push;
use super::sets::RefSets;

pub(super) fn collect(fsm: &Fsm, ir: &CoreIr) -> RefFailures {
    let refs = RefSets::new(fsm, ir);
    let mut failures = RefFailures::new();
    for transition in &fsm.transitions {
        collect_transition(fsm, transition, &refs, &mut failures);
    }
    failures
}

fn collect_transition(
    fsm: &Fsm,
    transition: &Transition,
    refs: &RefSets,
    failures: &mut RefFailures,
) {
    let subject = transition_subject(&fsm.id, &transition.id);
    push::state(
        &refs.states,
        &mut failures.unknown_from,
        &subject,
        "from",
        &transition.from,
    );
    push::state(
        &refs.states,
        &mut failures.unknown_to,
        &subject,
        "to",
        &transition.to,
    );
    push::event(
        &refs.events,
        &mut failures.unknown_events,
        &subject,
        transition,
    );
    push::refs(
        &refs.guards,
        &mut failures.unknown_guard_action,
        &subject,
        "guard",
        &transition.guards,
    );
    push::refs(
        &refs.actions,
        &mut failures.unknown_guard_action,
        &subject,
        "action",
        &transition.actions,
    );
    push::refs(
        &refs.subjects,
        &mut failures.unknown_requires,
        &subject,
        "requires",
        &transition.requires,
    );
}
