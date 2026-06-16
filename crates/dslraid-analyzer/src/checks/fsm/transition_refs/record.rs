use serde_json::Value;

use super::state_record;
use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(super) fn from_state(builder: &mut ReportBuilder, failures: &[Value]) {
    state_record::from_state(builder, failures);
}

pub(super) fn to_state(builder: &mut ReportBuilder, failures: &[Value]) {
    state_record::to_state(builder, failures);
}

pub(super) fn event(builder: &mut ReportBuilder, failures: &[Value]) {
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

pub(super) fn guard_action(builder: &mut ReportBuilder, failures: &[Value]) {
    record_collection_check(
        builder,
        CollectionCheck {
            proposition: "V017",
            assertion: "assertion:guard.references_existing_capability",
            code: "GUA017",
            layer: "guard_action",
            predicate: "guard_references_existing_capability",
            severity: "error",
            failures,
            pass_message: "All guard/action references resolve inside their FSM.",
            fail_message: "Some guard/action references do not resolve inside their FSM.",
            suggestion: "Declare the guard or action before referencing it.",
        },
    );
}

pub(super) fn requires(builder: &mut ReportBuilder, failures: &[Value]) {
    record_collection_check(
        builder,
        CollectionCheck {
            proposition: "V018",
            assertion: "assertion:action.uses_allowed_capability",
            code: "ACT018",
            layer: "guard_action",
            predicate: "action_uses_allowed_capability",
            severity: "error",
            failures,
            pass_message: "All transition requirements resolve to semantic subjects.",
            fail_message: "Some transition requirements do not resolve.",
            suggestion:
                "Reference an existing policy, capability, constraint, or semantic subject.",
        },
    );
}
