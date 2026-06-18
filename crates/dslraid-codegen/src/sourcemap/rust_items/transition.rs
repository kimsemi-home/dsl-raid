use dslraid_core::{transition_subject, Artifact, Fsm, Transition};

use crate::names::rust_type;

use super::super::generated::Index;

pub(super) fn add_transition(
    artifact: &Artifact,
    index: &mut Index,
    fsm: &Fsm,
    transition: &Transition,
    start: usize,
    end: usize,
    lines: &[&str],
) {
    let pattern = rust_transition_pattern(transition);
    if let Some(line) = super::super::lines::find(lines, start, end, &pattern) {
        super::super::push::generated(
            index,
            artifact,
            transition_subject(&fsm.id, &transition.id),
            line,
            line,
        );
    }
}

fn rust_transition_pattern(transition: &Transition) -> String {
    let event = transition
        .on
        .as_ref()
        .map(|event| format!("Some(\"{}\")", event))
        .unwrap_or_else(|| "None".to_string());
    format!("State::{}, {}) => Some", rust_type(&transition.from), event)
}
