use dslraid_core::{state_subject, Fsm};
use serde_json::json;

use crate::builder::{AssertionSpec, ReportBuilder};

pub(crate) fn check(fsm: &Fsm, builder: &mut ReportBuilder) {
    let missing = missing_sources(fsm);
    let passed = missing.is_empty();
    builder.record(AssertionSpec {
        proposition: "V033",
        assertion: "assertion:traceability.state_source_location_exists",
        code: "TRC033",
        layer: "traceability",
        predicate: "state_source_location_exists",
        severity: "warning",
        status: if passed { "passed" } else { "warning" },
        subjects: missing.clone(),
        evidence: json!({ "fsm": fsm.id, "states_without_source": missing }),
        message: Some(message(passed).to_string()),
        suggestion: Some("Attach defined_at to each state or to the parent FSM.".to_string()),
    });
}

fn missing_sources(fsm: &Fsm) -> Vec<String> {
    fsm.states
        .iter()
        .filter(|state| state.defined_at.is_none() && fsm.defined_at.is_none())
        .map(|state| state_subject(&fsm.id, &state.id))
        .collect()
}

fn message(passed: bool) -> &'static str {
    if passed {
        "Every state has a direct or FSM-level source location."
    } else {
        "Some states do not have a source location."
    }
}
