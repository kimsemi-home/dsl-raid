use dslraid_core::Fsm;
use serde_json::{json, Value};

use super::{groups, subjects};

pub(super) fn from_fsm(fsm: &Fsm) -> Vec<Value> {
    groups::from_fsm(fsm)
        .into_iter()
        .filter(|(_, transitions)| transitions.len() > 1)
        .map(|((state, event), transitions)| conflict(fsm, state, event, transitions))
        .collect()
}

fn conflict(fsm: &Fsm, state: String, event: String, transitions: Vec<String>) -> Value {
    json!({
        "fsm": fsm.id,
        "state": state,
        "event": event,
        "transitions": subjects::transitions(fsm, &transitions)
    })
}
