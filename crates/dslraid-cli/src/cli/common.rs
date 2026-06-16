use super::OutputFormat;
use clap::{Args, Subcommand};
use dslraid_core::CORE_SCHEMA_PATH;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub(crate) struct InitArgs {
    #[arg(default_value = ".dslraid.json")]
    pub(crate) out: PathBuf,
}

#[derive(Debug, Args)]
pub(crate) struct NormalizeArgs {
    pub(crate) input: PathBuf,
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct MigrateArgs {
    pub(crate) input: PathBuf,
    #[arg(long)]
    pub(crate) from: String,
    #[arg(long)]
    pub(crate) to: String,
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct ValidateArgs {
    pub(crate) input: PathBuf,
    #[arg(long, default_value = CORE_SCHEMA_PATH)]
    pub(crate) schema: PathBuf,
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub(crate) format: OutputFormat,
    #[arg(long = "deny")]
    pub(crate) deny: Vec<String>,
}

#[derive(Debug, Args)]
pub(crate) struct SchemaArgs {
    #[command(subcommand)]
    pub(crate) command: SchemaCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum SchemaCommand {
    Validate { schema: PathBuf, input: PathBuf },
}

#[derive(Debug, Args)]
pub(crate) struct GoldenArgs {
    #[command(subcommand)]
    pub(crate) command: GoldenCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum GoldenCommand {
    Check { path: PathBuf },
    Update { path: PathBuf },
}
