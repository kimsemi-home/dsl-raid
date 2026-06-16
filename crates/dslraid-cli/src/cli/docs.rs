use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Args)]
pub(crate) struct DocArgs {
    #[command(subcommand)]
    pub(crate) command: DocCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum DocCommand {
    Generate {
        input: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Check {
        input: PathBuf,
        #[arg(long)]
        golden: PathBuf,
    },
    Cli {
        #[command(subcommand)]
        command: CliDocCommand,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum CliDocCommand {
    Generate {
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Check {
        #[arg(long)]
        golden: PathBuf,
    },
}
