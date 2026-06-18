mod message;
mod reachable;
mod unreachable;

use dslraid_core::Fsm;
use serde_json::json;

use crate::builder::{AssertionSpec, ReportBuilder};

pub(crate) fn check(fsm: &Fsm, builder: &mut ReportBuilder) {
    let unreachable = unreachable::states(fsm);
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
        message: Some(message::from_passed(passed).to_string()),
        suggestion: Some(
            "Connect the state from the initial path or hide it through an explicit projection policy."
                .to_string(),
        ),
    });
}
