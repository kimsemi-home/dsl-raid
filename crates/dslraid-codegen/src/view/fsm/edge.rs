mod label;
mod lookup;
mod style;

use anyhow::Result;
use dslraid_core::{transition_subject, Fsm, Transition};

use super::diagnostic::DiagnosticMarks;
use super::edge_route::route;
use super::ids::layout_transition_id;
use crate::view::{ViewEdge, ViewNode};

pub(crate) fn transition_edge(
    fsm: &Fsm,
    transition: &Transition,
    nodes: &[ViewNode],
    diagnostics: &DiagnosticMarks,
) -> Result<ViewEdge> {
    let from_node = lookup::node_for(fsm, transition, nodes, &transition.from)?;
    let to_node = lookup::node_for(fsm, transition, nodes, &transition.to)?;
    let subject = transition_subject(&fsm.id, &transition.id);
    Ok(ViewEdge {
        id: layout_transition_id(fsm, &transition.id),
        subject: subject.clone(),
        from: from_node.id.clone(),
        to: to_node.id.clone(),
        label: Some(label::transition_label(transition)),
        route: route(from_node, to_node),
        style: Some(style::edge_style(transition, diagnostics.tone(&subject))),
    })
}
