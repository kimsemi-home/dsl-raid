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
    push::refs(
        &refs.guards,
        &mut failures.unknown_guard_action,
        subject,
        "guard",
        &transition.guards,
    );
    push::refs(
        &refs.actions,
        &mut failures.unknown_guard_action,
        subject,
        "action",
        &transition.actions,
    );
}
