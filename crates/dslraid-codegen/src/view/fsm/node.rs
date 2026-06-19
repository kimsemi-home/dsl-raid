mod badges;
mod layout;
mod style;

use dslraid_core::{state_subject, Fsm, State};

use super::diagnostic::DiagnosticMarks;
use super::ids::layout_state_id;
use crate::view::ViewNode;

pub(crate) fn state_node(
    fsm: &Fsm,
    state: &State,
    index: usize,
    diagnostics: &DiagnosticMarks,
) -> ViewNode {
    let subject = state_subject(&fsm.id, &state.id);
    ViewNode {
        id: layout_state_id(fsm, &state.id),
        subject: subject.clone(),
        x: layout::x(index),
        y: layout::y(index),
        width: layout::WIDTH,
        height: layout::HEIGHT,
        label: state.id.clone(),
        badges: badges::state_badges(state, diagnostics.badge(&subject)),
        style: Some(style::state_style(state, diagnostics.tone(&subject))),
    }
}
