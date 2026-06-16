use super::OutputFormat;
use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Args)]
pub(crate) struct TraceArgs {
    #[command(subcommand)]
    pub(crate) command: TraceCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum TraceCommand {
    Import {
        input: PathBuf,
        #[arg(long)]
        design_ir: Option<PathBuf>,
        #[arg(long)]
        run_id: Option<String>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Check {
        trace: PathBuf,
        #[arg(long)]
        design_ir: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Debug, Args)]
pub(crate) struct CoverageArgs {
    #[command(subcommand)]
    pub(crate) command: CoverageCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum CoverageCommand {
    Build {
        #[arg(long)]
        trace: PathBuf,
        #[arg(long)]
        design_ir: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Check {
        coverage: PathBuf,
        #[arg(long)]
        design_ir: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}
