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
    push::event(
        &refs.events,
        &mut failures.unknown_events,
        subject,
        transition,
    );
}
