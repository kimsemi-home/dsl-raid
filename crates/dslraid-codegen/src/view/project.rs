use anyhow::{anyhow, Result};
use dslraid_core::CoreIr;

use super::{composition, fsm, ViewModel};

pub fn project_view(
    ir: &CoreIr,
    projection_id: Option<&str>,
    core_path: impl Into<String>,
) -> Result<ViewModel> {
    let projection = ir
        .find_projection(projection_id)
        .ok_or_else(|| anyhow!("projection not found"))?;

    match projection.source.as_str() {
        source if source.starts_with("fsm:") => {
            let fsm = ir
                .find_fsm(source)
                .ok_or_else(|| anyhow!("projection source {} is not an FSM", source))?;
            fsm::build_fsm_view(ir, projection, fsm, core_path.into())
        }
        source if source.starts_with("composition:") => {
            let composition = ir
                .find_composition(source)
                .ok_or_else(|| anyhow!("projection source {} is not a composition", source))?;
            composition::build(ir, projection, composition, core_path.into())
        }
        source => Err(anyhow!("unsupported projection source {}", source)),
    }
}
