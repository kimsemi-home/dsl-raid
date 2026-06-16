use dslraid_core::Fsm;
use serde_json::json;

use crate::builder::{AssertionSpec, ReportBuilder};

pub(crate) fn check(fsm: &Fsm, builder: &mut ReportBuilder) {
    builder.record(AssertionSpec {
        proposition: "V006",
        assertion: "assertion:fsm.has_state",
        code: "FSM006",
        layer: "fsm",
        predicate: "fsm_has_state",
        severity: "error",
        status: if fsm.states.is_empty() {
            "failed"
        } else {
            "passed"
        },
        subjects: vec![fsm.id.clone()],
        evidence: json!({ "fsm": fsm.id, "state_count": fsm.states.len() }),
        message: Some(format!("{} has {} states.", fsm.name, fsm.states.len())),
        suggestion: Some("Define at least one state for every complete FSM.".to_string()),
    });
}
