use dslraid_core::{state_subject, transition_subject, Artifact, Fsm, Transition};

use crate::names::go_type;

use super::generated::Index;

pub(super) fn add_fsm(
    artifact: &Artifact,
    index: &mut Index,
    fsm: &Fsm,
    start: usize,
    end: usize,
    lines: &[&str],
) {
    let type_name = go_type(&fsm.name);
    super::push::generated(index, artifact, fsm.id.clone(), start, end);
    for state in &fsm.states {
        let pattern = format!("{type_name}State{} ", go_type(&state.id));
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
    let Some(case_line) = find_case(lines, fsm, transition, start, end) else {
        return;
    };
    let next_case = super::lines::find(lines, case_line + 1, end, "\tcase ").unwrap_or(end + 1);
    let pattern = format!("if event == \"{}\"", transition.on.as_deref().unwrap_or(""));
    if let Some(line) = super::lines::find(lines, case_line, next_case - 1, &pattern) {
        super::push::generated(
            index,
            artifact,
            transition_subject(&fsm.id, &transition.id),
            line,
            line + 2,
        );
    }
}

fn find_case(
    lines: &[&str],
    fsm: &Fsm,
    transition: &Transition,
    start: usize,
    end: usize,
) -> Option<usize> {
    let case = format!(
        "case {}State{}:",
        go_type(&fsm.name),
        go_type(&transition.from)
    );
    super::lines::find(lines, start, end, &case)
}
