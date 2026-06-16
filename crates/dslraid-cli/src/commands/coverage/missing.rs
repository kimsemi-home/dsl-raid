use dslraid_core::{state_subject, transition_subject, CoreIr};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn push_missing_design_subjects(
    ir: &CoreIr,
    covered_subjects: &BTreeSet<String>,
    issues: &mut Vec<Value>,
) {
    for fsm in &ir.fsms {
        for state in &fsm.states {
            push_missing_subject(
                covered_subjects,
                issues,
                state_subject(&fsm.id, &state.id),
                "Coverage overlay is missing a state subject.",
            );
        }
        for transition in &fsm.transitions {
            push_missing_subject(
                covered_subjects,
                issues,
                transition_subject(&fsm.id, &transition.id),
                "Coverage overlay is missing a transition subject.",
            );
        }
    }
}

fn push_missing_subject(
    covered_subjects: &BTreeSet<String>,
    issues: &mut Vec<Value>,
    subject: String,
    message: &str,
) {
    if covered_subjects.contains(&subject) {
        return;
    }
    issues.push(serde_json::json!({
        "code": "COV002",
        "subject": subject,
        "message": message
    }));
}
