use dslraid_core::{state_subject, Fsm};
use serde_json::json;

use crate::builder::{AssertionSpec, ReportBuilder};

pub(crate) fn check(fsm: &Fsm, builder: &mut ReportBuilder) {
    let initial_states = initial_states(fsm);
    let initial_count = initial_states.len();
    let mut subjects = vec![fsm.id.clone()];
    subjects.extend(
        initial_states
            .iter()
            .map(|state| state_subject(&fsm.id, state)),
    );
    builder.record(AssertionSpec {
        proposition: "V007",
        assertion: "assertion:fsm.initial_exactly_one",
        code: "FSM007",
        layer: "fsm",
        predicate: "exactly_one_initial_state",
        severity: "error",
        status: if initial_count == 1 {
            "passed"
        } else {
            "failed"
        },
        subjects,
        evidence: json!({
            "fsm": fsm.id,
            "initial_count": initial_count,
            "expected": 1,
            "initial_states": initial_states
        }),
        message: Some(format!(
            "{} has {} initial states.",
            fsm.name, initial_count
        )),
        suggestion: Some("Mark exactly one state as initial.".to_string()),
    });
}

fn initial_states(fsm: &Fsm) -> Vec<String> {
    fsm.states
        .iter()
        .filter(|state| state.initial)
        .map(|state| state.id.clone())
        .collect()
}
