use anyhow::Result;
use dslraid_core::Fsm;

use super::diagnostic::DiagnosticMarks;
use super::edge::transition_edge;
use super::node::state_node;
use crate::view::{ViewEdge, ViewNode};

pub(crate) fn state_nodes(fsm: &Fsm, diagnostics: &DiagnosticMarks) -> Vec<ViewNode> {
    fsm.states
        .iter()
        .enumerate()
        .map(|(index, state)| state_node(fsm, state, index, diagnostics))
        .collect()
}

pub(crate) fn transition_edges(
    fsm: &Fsm,
    nodes: &[ViewNode],
    diagnostics: &DiagnosticMarks,
) -> Result<Vec<ViewEdge>> {
    fsm.transitions
        .iter()
        .map(|transition| transition_edge(fsm, transition, nodes, diagnostics))
        .collect()
}
