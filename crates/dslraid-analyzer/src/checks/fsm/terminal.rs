use std::collections::BTreeSet;

use dslraid_core::{transition_subject, Fsm};
use serde_json::{json, Value};

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(crate) fn check(fsm: &Fsm, builder: &mut ReportBuilder) {
    let outgoing = outgoing_from_terminal(fsm);
    record_collection_check(
        builder,
        CollectionCheck {
            proposition: "V011",
            assertion: "assertion:fsm.terminal_has_no_outgoing",
            code: "FSM011",
            layer: "fsm",
            predicate: "terminal_has_no_outgoing",
            severity: "error",
            failures: &outgoing,
            pass_message: "Terminal states do not have outgoing transitions.",
            fail_message: "A terminal state has outgoing transitions.",
            suggestion:
                "Move the outgoing transition to a non-terminal state or remove terminal=true.",
        },
    );
}

fn outgoing_from_terminal(fsm: &Fsm) -> Vec<Value> {
    let terminal: BTreeSet<_> = fsm
        .states
        .iter()
        .filter(|state| state.terminal)
        .map(|state| state.id.as_str())
        .collect();
    fsm.transitions
        .iter()
        .filter(|transition| terminal.contains(transition.from.as_str()))
        .map(|transition| {
            json!({
                "transition": transition_subject(&fsm.id, &transition.id),
                "from": transition.from,
                "to": transition.to
            })
        })
        .collect()
}
