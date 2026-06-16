use serde_json::Value;

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(super) fn from_state(builder: &mut ReportBuilder, failures: &[Value]) {
    record_collection_check(builder, check("from", failures));
}

pub(super) fn to_state(builder: &mut ReportBuilder, failures: &[Value]) {
    record_collection_check(builder, check("to", failures));
}

fn check<'a>(direction: &'static str, failures: &'a [Value]) -> CollectionCheck<'a> {
    CollectionCheck {
        proposition: if direction == "from" { "V008" } else { "V009" },
        assertion: assertion(direction),
        code: if direction == "from" {
            "FSM008"
        } else {
            "FSM009"
        },
        layer: "fsm",
        predicate: if direction == "from" {
            "transition_from_state_exists"
        } else {
            "transition_to_state_exists"
        },
        severity: "error",
        failures,
        pass_message: pass_message(direction),
        fail_message: fail_message(direction),
        suggestion: "Use a state ID defined in the same FSM.",
    }
}

fn assertion(direction: &str) -> &'static str {
    if direction == "from" {
        "assertion:fsm.transition_from_exists"
    } else {
        "assertion:fsm.transition_target_exists"
    }
}

fn pass_message(direction: &str) -> &'static str {
    if direction == "from" {
        "All transition.from states resolve inside their FSM."
    } else {
        "All transition.to states resolve inside their FSM."
    }
}

fn fail_message(direction: &str) -> &'static str {
    if direction == "from" {
        "Some transition.from states do not resolve inside their FSM."
    } else {
        "Some transition.to states do not resolve inside their FSM."
    }
}
