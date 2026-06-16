use anyhow::Result;
use dslraid_core::{CoreIr, Fsm, Projection};

use super::edge::transition_edge;
use super::node::state_node;
use super::panels::{fsm_panel, state_panel, transition_panel};
use super::{Layout, ViewEdge, ViewModel, ViewNode, ViewSource};

pub(crate) const VIEW_VERSION: &str = "0.1.0";

pub(crate) fn build_fsm_view(
    ir: &CoreIr,
    projection: &Projection,
    fsm: &Fsm,
    core_path: String,
) -> Result<ViewModel> {
    let nodes = state_nodes(fsm);
    let edges = transition_edges(fsm, &nodes)?;
    let panels = inspector_panels(ir, fsm);
    Ok(ViewModel {
        view_version: VIEW_VERSION.to_string(),
        source: ViewSource {
            core_ir: core_path,
            projection: projection.id.clone(),
            index: None,
            hash: dslraid_core::sha256_json(ir).ok(),
        },
        layout: Layout {
            engine: "manual".to_string(),
            version: VIEW_VERSION.to_string(),
        },
        nodes,
        edges,
        inspector_panels: panels,
    })
}

fn state_nodes(fsm: &Fsm) -> Vec<ViewNode> {
    fsm.states
        .iter()
        .enumerate()
        .map(|(index, state)| state_node(fsm, state, index))
        .collect()
}

fn transition_edges(fsm: &Fsm, nodes: &[ViewNode]) -> Result<Vec<ViewEdge>> {
    fsm.transitions
        .iter()
        .map(|transition| transition_edge(fsm, transition, nodes))
        .collect()
}

fn inspector_panels(ir: &CoreIr, fsm: &Fsm) -> Vec<super::InspectorPanel> {
    let mut panels = Vec::new();
    panels.extend(fsm.states.iter().map(|state| {
        state_panel(
            fsm,
            &state.id,
            &dslraid_core::state_subject(&fsm.id, &state.id),
        )
    }));
    panels.extend(fsm.transitions.iter().map(|transition| {
        transition_panel(
            fsm,
            transition,
            &dslraid_core::transition_subject(&fsm.id, &transition.id),
        )
    }));
    panels.push(fsm_panel(ir, fsm));
    panels
}
