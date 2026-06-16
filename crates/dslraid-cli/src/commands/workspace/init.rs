use anyhow::{bail, Result};
use std::path::Path;

pub(crate) fn run(out: &Path) -> Result<()> {
    if out.exists() {
        bail!("{} already exists", out.display());
    }
    let template = serde_json::json!({
        "ir_version": "0.1.0",
        "project": {
            "id": "dslraid-project",
            "name": "DSLRaid Project",
            "visibility": "public"
        },
        "contexts": [],
        "requirements": [],
        "capabilities": [],
        "policies": [],
        "commands": [],
        "fsms": [],
        "compositions": [],
        "projections": [],
        "derivations": [],
        "artifacts": [],
        "diagnostics": []
    });
    crate::write_bytes(out, serde_json::to_string_pretty(&template)?.as_bytes())?;
    println!("created {}", out.display());
    Ok(())
}
