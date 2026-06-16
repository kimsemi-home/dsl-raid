mod artifacts;
mod derivations;
mod model;
mod preserve;
mod tool;

use anyhow::Result;
use dslraid_core::{load_core_ir, sha256_json};
use std::path::Path;

pub(super) fn run(input: &Path, out: Option<&Path>) -> Result<()> {
    let lock_path = out
        .map(Path::to_path_buf)
        .unwrap_or_else(|| super::lock::inferred_lock_path(input));
    let lock = build(input)?;
    let mut bytes = serde_json::to_vec_pretty(&lock)?;
    bytes.push(b'\n');
    crate::write_bytes(&lock_path, &bytes)?;
    println!("updated {}", lock_path.display());
    Ok(())
}

fn build(input: &Path) -> Result<model::LockFile> {
    let ir = load_core_ir(input)?;
    let current_hash = sha256_json(&ir)?;
    let lock_path = super::lock::inferred_lock_path(input);
    let preserved = preserve::load(&lock_path)?;
    Ok(model::LockFile {
        lock_version: "0.1.0",
        core: model::CoreRecord::new(input, &current_hash),
        resolver: tool::dslraid(),
        resolved_refs: preserved.resolved_refs,
        derivations: derivations::records(&ir, &current_hash),
        artifacts: artifacts::records(&ir, input, &current_hash, &preserved.artifacts),
        golden: preserved.golden,
        compat: preserved.compat,
    })
}
