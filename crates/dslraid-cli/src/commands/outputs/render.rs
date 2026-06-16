use anyhow::{anyhow, Result};
use dslraid_codegen::{project_view, render_svg, ViewModel};
use dslraid_core::load_core_ir;
use std::fs;
use std::path::Path;

pub(crate) fn run(
    input: &Path,
    projection: Option<&str>,
    format: crate::RenderFormat,
    out: Option<&Path>,
) -> Result<()> {
    let ir = load_core_ir(input)?;
    let view = project_view(&ir, projection, input.display().to_string())?;
    match out {
        Some(path) if path.extension().is_none() || path.is_dir() => {
            render_directory(input, path, &view)
        }
        Some(path) => match format {
            crate::RenderFormat::Svg => crate::write_bytes(path, render_svg(&view).as_bytes()),
            crate::RenderFormat::Json => {
                crate::write_bytes(path, serde_json::to_string_pretty(&view)?.as_bytes())
            }
        },
        None => print_rendered(format, &view),
    }
}

fn render_directory(input: &Path, path: &Path, view: &ViewModel) -> Result<()> {
    fs::create_dir_all(path)?;
    let stem = input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("dslraid");
    let view_path = path.join(format!("{stem}.view.json"));
    let svg_path = path.join(format!("{stem}.svg"));
    let core_path = path.join(
        input
            .file_name()
            .ok_or_else(|| anyhow!("input has no file name"))?,
    );
    crate::write_bytes(&view_path, serde_json::to_string_pretty(view)?.as_bytes())?;
    crate::write_bytes(&svg_path, render_svg(view).as_bytes())?;
    fs::copy(input, core_path)?;
    println!("rendered {}", path.display());
    Ok(())
}

fn print_rendered(format: crate::RenderFormat, view: &ViewModel) -> Result<()> {
    match format {
        crate::RenderFormat::Svg => {
            print!("{}", render_svg(view));
            Ok(())
        }
        crate::RenderFormat::Json => {
            println!("{}", serde_json::to_string_pretty(view)?);
            Ok(())
        }
    }
}
