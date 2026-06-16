use std::collections::{BTreeSet, VecDeque};

use dslraid_core::{state_subject, Fsm};
use serde_json::json;

use crate::builder::{AssertionSpec, ReportBuilder};

pub(crate) fn check(fsm: &Fsm, builder: &mut ReportBuilder) {
    let unreachable = unreachable_states(fsm);
    let passed = unreachable.is_empty();
    builder.record(AssertionSpec {
        proposition: "V012",
        assertion: "assertion:fsm.states_reachable",
        code: "FSM012",
        layer: "fsm",
        predicate: "states_reachable_from_initial",
        severity: "warning",
        status: if passed { "passed" } else { "warning" },
        subjects: unreachable.clone(),
        evidence: json!({ "fsm": fsm.id, "unreachable_states": unreachable }),
        message: Some(message(passed).to_string()),
        suggestion: Some(
            "Connect the state from the initial path or hide it through an explicit projection policy."
                .to_string(),
        ),
    });
}

fn unreachable_states(fsm: &Fsm) -> Vec<String> {
    let reachable = reachable_states(fsm);
    fsm.states
        .iter()
        .filter(|state| !reachable.contains(&state.id))
        .map(|state| state_subject(&fsm.id, &state.id))
        .collect()
}

fn reachable_states(fsm: &Fsm) -> BTreeSet<String> {
    let mut reachable = BTreeSet::new();
    let Some(initial) = fsm.states.iter().find(|state| state.initial) else {
        return reachable;
    };
    let mut queue = VecDeque::from([initial.id.clone()]);
    while let Some(state) = queue.pop_front() {
        if !reachable.insert(state.clone()) {
            continue;
        }
        queue.extend(
            fsm.transitions
                .iter()
                .filter(|transition| transition.from == state)
                .map(|transition| transition.to.clone()),
        );
    }
    reachable
}

fn message(passed: bool) -> &'static str {
    if passed {
        "All states are reachable from the initial state."
    } else {
        "Some states are not reachable from the initial state."
    }
}
