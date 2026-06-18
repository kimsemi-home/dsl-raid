use dslraid_core::{transition_subject, Artifact, Fsm, Transition};

use crate::names::go_type;

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
    let Some(case_line) = find_case(lines, fsm, transition, start, end) else {
        return;
    };
    let next_case =
        super::super::lines::find(lines, case_line + 1, end, "\tcase ").unwrap_or(end + 1);
    let pattern = format!("if event == \"{}\"", transition.on.as_deref().unwrap_or(""));
    if let Some(line) = super::super::lines::find(lines, case_line, next_case - 1, &pattern) {
        super::super::push::generated(
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
    super::super::lines::find(lines, start, end, &case)
}
