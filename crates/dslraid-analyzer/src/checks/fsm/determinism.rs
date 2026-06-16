use std::collections::BTreeMap;

use dslraid_core::{transition_subject, Fsm};
use serde_json::{json, Value};

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(crate) fn check(fsm: &Fsm, builder: &mut ReportBuilder) {
    let conflicts = conflicts(fsm);
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

fn conflicts(fsm: &Fsm) -> Vec<Value> {
    grouped_transitions(fsm)
        .into_iter()
        .filter(|(_, transitions)| transitions.len() > 1)
        .map(|((state, event), transitions)| {
            json!({
                "fsm": fsm.id,
                "state": state,
                "event": event,
                "transitions": transition_subjects(fsm, &transitions)
            })
        })
        .collect()
}

fn grouped_transitions(fsm: &Fsm) -> BTreeMap<(String, String), Vec<String>> {
    let mut by_key = BTreeMap::new();
    for transition in &fsm.transitions {
        let event = transition
            .on
            .clone()
            .unwrap_or_else(|| "epsilon".to_string());
        by_key
            .entry((transition.from.clone(), event))
            .or_insert_with(Vec::new)
            .push(transition.id.clone());
    }
    by_key
}

fn transition_subjects(fsm: &Fsm, transitions: &[String]) -> Vec<String> {
    transitions
        .iter()
        .map(|transition| transition_subject(&fsm.id, transition))
        .collect()
}
