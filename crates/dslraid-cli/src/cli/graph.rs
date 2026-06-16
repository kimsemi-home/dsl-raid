use super::{DiffFormat, OutputFormat};
use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub(crate) struct ComposeArgs {
    pub(crate) input: PathBuf,
    #[arg(long)]
    pub(crate) composition: Option<String>,
    #[arg(long, default_value = "diagnostics-only")]
    pub(crate) materialize: String,
    #[arg(long, default_value_t = 5000)]
    pub(crate) limit: usize,
    #[arg(long)]
    pub(crate) focus: Option<String>,
    #[arg(long, default_value_t = 1)]
    pub(crate) depth: usize,
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub(crate) format: OutputFormat,
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct DiffArgs {
    pub(crate) base: PathBuf,
    pub(crate) head: PathBuf,
    #[arg(long, value_enum, default_value_t = DiffFormat::Text)]
    pub(crate) format: DiffFormat,
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct QueryArgs {
    pub(crate) input: PathBuf,
    pub(crate) expression: String,
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub(crate) format: OutputFormat,
}
