use serde_json::Value;

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(super) fn record(builder: &mut ReportBuilder, failures: &[Value]) {
    record_collection_check(
        builder,
        CollectionCheck {
            proposition: "V010",
            assertion: "assertion:fsm.transition_event_exists",
            code: "FSM010",
            layer: "fsm",
            predicate: "transition_event_exists",
            severity: "error",
            failures,
            pass_message: "All transition events resolve inside their FSM.",
            fail_message: "Some transition events do not resolve inside their FSM.",
            suggestion: "Declare the event or remove the transition event reference.",
        },
    );
}
