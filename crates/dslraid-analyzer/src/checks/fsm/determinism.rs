mod conflicts;
mod groups;
mod subjects;

use dslraid_core::Fsm;

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(crate) fn check(fsm: &Fsm, builder: &mut ReportBuilder) {
    let conflicts = conflicts::from_fsm(fsm);
    record_collection_check(
        builder,
        CollectionCheck {
            proposition: "V015",
            assertion: "assertion:fsm.event_handling_deterministic",
            code: "FSM015",
            layer: "fsm",
            predicate: "event_handling_deterministic",
            severity: "error",
            failures: &conflicts,
            pass_message: "Transitions are deterministic by state and event.",
            fail_message: "Multiple transitions handle the same event from the same state.",
            suggestion: "Add mutually exclusive guards or merge the transitions.",
        },
    );
}
