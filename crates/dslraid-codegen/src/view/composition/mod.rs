mod edge;
mod expand;
mod model;
mod node;
mod panel;
mod panel_edge;
mod panel_tail;
mod reachable;
mod reachable_tail;
mod tuple;

use anyhow::{anyhow, Result};
use dslraid_core::{Composition, CoreIr, Fsm, Projection};

use super::{Layout, ViewModel, ViewSource};

pub(crate) fn build(
    ir: &CoreIr,
    projection: &Projection,
    composition: &Composition,
    core_path: String,
) -> Result<ViewModel> {
    let fsms = input_fsms(ir, composition)?;
    let result = reachable::materialize(composition, &fsms);
    let nodes = node::nodes(&result.nodes);
    let edges = edge::edges(&result.edges, &nodes);
    let panels = panel::panels(composition, &result);
    Ok(ViewModel {
        view_version: super::VIEW_VERSION.to_string(),
        source: ViewSource {
            core_ir: core_path,
            projection: projection.id.clone(),
            index: None,
            hash: dslraid_core::sha256_json(ir).ok(),
        },
        layout: Layout {
            engine: "bounded-reachable-product".to_string(),
            version: super::VIEW_VERSION.to_string(),
        },
        nodes,
        edges,
        inspector_panels: panels,
    })
}

fn input_fsms<'a>(ir: &'a CoreIr, composition: &Composition) -> Result<Vec<&'a Fsm>> {
    composition
        .inputs
        .iter()
        .map(|id| {
            ir.find_fsm(id)
                .ok_or_else(|| anyhow!("unknown FSM input {id}"))
        })
        .collect()
}
