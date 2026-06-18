use super::model::DiffEndpoint;
use anyhow::Result;
use dslraid_core::{sha256_json, CoreIr};
use std::path::Path;

pub(super) fn endpoint(path: &Path, ir: &CoreIr) -> Result<DiffEndpoint> {
    Ok(DiffEndpoint {
        path: path.display().to_string(),
        hash: sha256_json(ir)?,
        ir_version: ir.ir_version.clone(),
    })
}
