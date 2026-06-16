use crate::{
    commands, ArtifactArgs, ArtifactCommand, CoverageArgs, CoverageCommand, TraceArgs, TraceCommand,
};
use anyhow::Result;

pub(super) fn trace(args: TraceArgs) -> Result<()> {
    match args.command {
        TraceCommand::Import {
            input,
            design_ir,
            run_id,
            out,
        } => commands::trace::import(
            &input,
            design_ir.as_deref(),
            run_id.as_deref(),
            out.as_deref(),
        ),
        TraceCommand::Check {
            trace,
            design_ir,
            format,
        } => commands::trace::check(&trace, &design_ir, format),
    }
}

pub(super) fn coverage(args: CoverageArgs) -> Result<()> {
    match args.command {
        CoverageCommand::Build {
            trace,
            design_ir,
            out,
        } => commands::coverage::build(&trace, &design_ir, out.as_deref()),
        CoverageCommand::Check {
            coverage,
            design_ir,
            format,
        } => commands::coverage::check(&coverage, &design_ir, format),
    }
}

pub(super) fn artifact(args: ArtifactArgs) -> Result<()> {
    match args.command {
        ArtifactCommand::Verify {
            input,
            lock,
            format,
        } => commands::artifact::verify(&input, lock.as_deref(), format),
    }
}
