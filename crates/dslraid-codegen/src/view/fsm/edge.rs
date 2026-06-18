use anyhow::{anyhow, Result};
use dslraid_core::{transition_subject, Fsm, Transition};

use super::diagnostic::DiagnosticMarks;
use super::edge_route::route;
use super::ids::layout_transition_id;
use crate::view::{StyleToken, ViewEdge, ViewNode};

pub(crate) fn transition_edge(
    fsm: &Fsm,
    transition: &Transition,
    nodes: &[ViewNode],
    diagnostics: &DiagnosticMarks,
) -> Result<ViewEdge> {
    let from_node = node_for(fsm, transition, nodes, &transition.from)?;
    let to_node = node_for(fsm, transition, nodes, &transition.to)?;
    let subject = transition_subject(&fsm.id, &transition.id);
    Ok(ViewEdge {
        id: layout_transition_id(fsm, &transition.id),
        subject: subject.clone(),
        from: from_node.id.clone(),
        to: to_node.id.clone(),
        label: Some(
            transition
                .on
                .clone()
                .unwrap_or_else(|| "epsilon".to_string()),
        ),
        route: route(from_node, to_node),
        style: Some(edge_style(transition, diagnostics.tone(&subject))),
    })
}

fn node_for<'a>(
    fsm: &Fsm,
    transition: &Transition,
    nodes: &'a [ViewNode],
    state_id: &str,
) -> Result<&'a ViewNode> {
    let index = fsm
        .states
        .iter()
        .position(|state| state.id == state_id)
        .ok_or_else(|| unknown_state(transition, state_id))?;
    Ok(&nodes[index])
}

fn unknown_state(transition: &Transition, state_id: &str) -> anyhow::Error {
    anyhow!(
        "transition {} has unknown state {}",
        transition.id,
        state_id
    )
}

fn edge_style(transition: &Transition, diagnostic: Option<&str>) -> StyleToken {
    let fallback = if transition.requires.is_empty() {
        "default"
    } else {
        "warning"
    };
    StyleToken {
        tone: diagnostic.unwrap_or(fallback).to_string(),
        emphasis: "normal".to_string(),
    }
}
