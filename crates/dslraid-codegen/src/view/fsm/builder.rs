use anyhow::Result;
use dslraid_core::{CoreIr, Fsm, Projection};

use super::diagnostic::DiagnosticMarks;
use super::panels::{fsm_panel, state_panel, transition_panel};
use super::scene::{state_nodes, transition_edges};
use crate::view::{InspectorPanel, Layout, ViewModel, ViewSource, VIEW_VERSION};

pub(crate) fn build_fsm_view(
    ir: &CoreIr,
    projection: &Projection,
    fsm: &Fsm,
    core_path: String,
) -> Result<ViewModel> {
    let diagnostics = DiagnosticMarks::from_ir(ir);
    let nodes = state_nodes(fsm, &diagnostics);
    let edges = transition_edges(fsm, &nodes, &diagnostics)?;
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

fn inspector_panels(ir: &CoreIr, fsm: &Fsm) -> Vec<InspectorPanel> {
    let mut panels = Vec::new();
    panels.extend(fsm.states.iter().map(|state| {
        state_panel(
            ir,
            fsm,
            &state.id,
            &dslraid_core::state_subject(&fsm.id, &state.id),
        )
    }));
    panels.extend(fsm.transitions.iter().map(|transition| {
        transition_panel(
            ir,
            fsm,
            transition,
            &dslraid_core::transition_subject(&fsm.id, &transition.id),
        )
    }));
    panels.push(fsm_panel(ir, fsm));
    panels
}
