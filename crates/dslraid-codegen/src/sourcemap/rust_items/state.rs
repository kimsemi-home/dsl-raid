use dslraid_core::{state_subject, Artifact, Fsm, State};

use crate::names::rust_type;

use super::super::generated::Index;

pub(super) fn add_state(
    artifact: &Artifact,
    index: &mut Index,
    fsm: &Fsm,
    state: &State,
    start: usize,
    end: usize,
    lines: &[&str],
) {
    let pattern = format!("    {},", rust_type(&state.id));
    if let Some(line) = super::super::lines::find(lines, start, end, &pattern) {
        super::super::push::generated(
            index,
            artifact,
            state_subject(&fsm.id, &state.id),
            line,
            line,
        );
    }
}
