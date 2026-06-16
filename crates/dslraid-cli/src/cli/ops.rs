use super::OutputFormat;
use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Args)]
pub(crate) struct ArtifactArgs {
    #[command(subcommand)]
    pub(crate) command: ArtifactCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum ArtifactCommand {
    Verify {
        input: PathBuf,
        #[arg(long)]
        lock: Option<PathBuf>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    Lock {
        #[command(subcommand)]
        command: ArtifactLockCommand,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum ArtifactLockCommand {
    Update {
        input: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Debug, Args)]
pub(crate) struct CompatArgs {
    #[command(subcommand)]
    pub(crate) command: CompatCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum CompatCommand {
    Check { input: PathBuf },
}
