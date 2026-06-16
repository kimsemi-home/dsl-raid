use dslraid_core::{state_subject, transition_subject, Artifact, Fsm, Transition};

use crate::names::rust_type;

use super::generated::Index;

pub(super) fn add_fsm(
    artifact: &Artifact,
    index: &mut Index,
    fsm: &Fsm,
    start: usize,
    end: usize,
    lines: &[&str],
) {
    super::push::generated(index, artifact, fsm.id.clone(), start, end);
    for state in &fsm.states {
        let pattern = format!("    {},", rust_type(&state.id));
        if let Some(line) = super::lines::find(lines, start, end, &pattern) {
            super::push::generated(
                index,
                artifact,
                state_subject(&fsm.id, &state.id),
                line,
                line,
            );
        }
    }
    for transition in &fsm.transitions {
        add_transition(artifact, index, fsm, transition, start, end, lines);
    }
}

fn add_transition(
    artifact: &Artifact,
    index: &mut Index,
    fsm: &Fsm,
    transition: &Transition,
    start: usize,
    end: usize,
    lines: &[&str],
) {
    let pattern = rust_transition_pattern(transition);
    if let Some(line) = super::lines::find(lines, start, end, &pattern) {
        super::push::generated(
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
