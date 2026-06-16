use anyhow::{anyhow, Result};
use dslraid_core::{transition_subject, Fsm, Transition};

use super::{layout_transition_id, Point, StyleToken, ViewEdge, ViewNode};

pub(crate) fn transition_edge(
    fsm: &Fsm,
    transition: &Transition,
    nodes: &[ViewNode],
) -> Result<ViewEdge> {
    let from_node = node_for(fsm, transition, nodes, &transition.from)?;
    let to_node = node_for(fsm, transition, nodes, &transition.to)?;
    Ok(ViewEdge {
        id: layout_transition_id(fsm, &transition.id),
        subject: transition_subject(&fsm.id, &transition.id),
        from: from_node.id.clone(),
        to: to_node.id.clone(),
        label: Some(
            transition
                .on
                .clone()
                .unwrap_or_else(|| "epsilon".to_string()),
        ),
        route: route(from_node, to_node),
        style: Some(edge_style(transition)),
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

fn route(from: &ViewNode, to: &ViewNode) -> Vec<Point> {
    vec![
        Point {
            x: from.x + from.width,
            y: from.y + from.height / 2.0,
        },
        Point {
            x: to.x,
            y: to.y + to.height / 2.0,
        },
    ]
}

fn edge_style(transition: &Transition) -> StyleToken {
    StyleToken {
        tone: if transition.requires.is_empty() {
            "default"
        } else {
            "warning"
        }
        .to_string(),
        emphasis: "normal".to_string(),
    }
}
