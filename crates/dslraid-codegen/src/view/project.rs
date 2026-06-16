use anyhow::{anyhow, Result};
use dslraid_core::CoreIr;

use super::{build_fsm_view, ViewModel};

pub fn project_view(
    ir: &CoreIr,
    projection_id: Option<&str>,
    core_path: impl Into<String>,
) -> Result<ViewModel> {
    let projection = ir
        .find_projection(projection_id)
        .ok_or_else(|| anyhow!("projection not found"))?;

    let fsm = match projection.source.as_str() {
        source if source.starts_with("fsm:") => ir
            .find_fsm(source)
            .ok_or_else(|| anyhow!("projection source {} is not an FSM", source))?,
        source => {
            return Err(anyhow!(
                "only FSM projections are implemented for MVP, got {}",
                source
            ))
        }
    };

    build_fsm_view(ir, projection, fsm, core_path.into())
}
