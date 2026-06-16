use crate::{commands, CodegenArgs, ExportArgs, ProjectArgs, RenderArgs};
use anyhow::Result;

pub(super) fn project(args: ProjectArgs) -> Result<()> {
    commands::outputs::project(&args.input, args.projection.as_deref(), args.out.as_deref())
}

pub(super) fn render(args: RenderArgs) -> Result<()> {
    commands::outputs::render(
        &args.input,
        args.projection.as_deref(),
        args.format,
        args.out.as_deref(),
    )
}

pub(super) fn codegen(args: CodegenArgs) -> Result<()> {
    commands::outputs::codegen(&args.input, args.target, args.out.as_deref())
}

pub(super) fn export(args: ExportArgs) -> Result<()> {
    commands::outputs::export(&args.input, args.target, args.out.as_deref())
}
