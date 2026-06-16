use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Args)]
pub(crate) struct DemoArgs {
    #[command(subcommand)]
    pub(crate) command: DemoCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum DemoCommand {
    /// Package deterministic assets consumed by the web viewer demo.
    Package {
        /// Canonical IR input file.
        input: PathBuf,
        /// Output directory for viewer-readable assets.
        #[arg(long)]
        out: PathBuf,
        /// Runtime trace to copy and convert into a coverage overlay.
        #[arg(long)]
        trace: Option<PathBuf>,
    },
}
