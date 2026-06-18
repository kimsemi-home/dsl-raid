use dslraid_core::Transition;

use super::failures::RefFailures;
use super::push;
use super::sets::RefSets;

pub(super) fn collect(
    refs: &RefSets,
    failures: &mut RefFailures,
    subject: &str,
    transition: &Transition,
) {
    push::state(
        &refs.states,
        &mut failures.unknown_from,
        subject,
        "from",
        &transition.from,
    );
    push::state(
        &refs.states,
        &mut failures.unknown_to,
        subject,
        "to",
        &transition.to,
    );
}
