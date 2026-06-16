use anyhow::{bail, Result};
use dslraid_codegen::{project_view, render_svg};
use dslraid_core::{CoreIr, VIEW_SCHEMA_PATH};
use std::fs;
use std::path::Path;

pub(super) fn check(input: &Path, ir: &CoreIr) -> Result<()> {
    let view = project_view(ir, Some("view:runtime"), input.display().to_string())?;
    let view_path = std::env::temp_dir().join(format!("dslraid-view-{}.json", std::process::id()));
    crate::write_bytes(&view_path, serde_json::to_string_pretty(&view)?.as_bytes())?;
    crate::schema_validate(Path::new(VIEW_SCHEMA_PATH), &view_path)?;
    fs::remove_file(&view_path).ok();

    let svg = render_svg(&view);
    if !svg.contains("<svg") || svg.len() < 200 {
        bail!("rendered SVG is unexpectedly empty");
    }
    Ok(())
}
