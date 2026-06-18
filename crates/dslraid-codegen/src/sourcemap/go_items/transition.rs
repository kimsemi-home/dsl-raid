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
    if let Some(line) = find_table_entry(lines, fsm, transition, start, end) {
        super::super::push::generated(
            index,
            artifact,
            transition_subject(&fsm.id, &transition.id),
            line,
            line,
        );
    }
}

fn find_table_entry(
    lines: &[&str],
    fsm: &Fsm,
    transition: &Transition,
    start: usize,
    end: usize,
) -> Option<usize> {
    let state = format!("{}State{}", go_type(&fsm.name), go_type(&transition.from));
    let target = table_target(fsm, transition);
    lines
        .iter()
        .enumerate()
        .skip(start.saturating_sub(1))
        .take(end.saturating_sub(start) + 1)
        .find(|(_, line)| line.contains(&state) && line.contains(&target))
        .map(|(index, _)| index + 1)
}

fn table_target(fsm: &Fsm, transition: &Transition) -> String {
    let event = transition.on.as_deref().unwrap_or("");
    format!(
        "\"{event}\": {}State{}",
        go_type(&fsm.name),
        go_type(&transition.to)
    )
}
