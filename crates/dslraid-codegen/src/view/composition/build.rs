use anyhow::Result;
use dslraid_core::{Composition, CoreIr, Projection};

use super::inputs::input_fsms;
use crate::view::{Layout, ViewModel, ViewSource};

pub(crate) fn build(
    ir: &CoreIr,
    projection: &Projection,
    composition: &Composition,
    core_path: String,
) -> Result<ViewModel> {
    let fsms = input_fsms(ir, composition)?;
    let result = super::reachable::materialize(composition, &fsms);
    let nodes = super::node::nodes(&result.nodes);
    let edges = super::edge::edges(&result.edges, &nodes);
    let panels = super::panel::panels(composition, &result);
    Ok(ViewModel {
        view_version: super::super::VIEW_VERSION.to_string(),
        source: ViewSource {
            core_ir: core_path,
            projection: projection.id.clone(),
            index: None,
            hash: dslraid_core::sha256_json(ir).ok(),
        },
        layout: Layout {
            engine: "bounded-reachable-product".to_string(),
            version: super::super::VIEW_VERSION.to_string(),
        },
        nodes,
        edges,
        inspector_panels: panels,
    })
}
