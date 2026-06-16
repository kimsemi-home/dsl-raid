use super::{CliCodegenTarget, CliExportTarget, RenderFormat};
use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub(crate) struct ProjectArgs {
    pub(crate) input: PathBuf,
    #[arg(long)]
    pub(crate) projection: Option<String>,
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct RenderArgs {
    pub(crate) input: PathBuf,
    #[arg(long)]
    pub(crate) projection: Option<String>,
    #[arg(long, value_enum, default_value_t = RenderFormat::Svg)]
    pub(crate) format: RenderFormat,
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct CodegenArgs {
    pub(crate) input: PathBuf,
    #[arg(long, value_enum)]
    pub(crate) target: CliCodegenTarget,
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct ExportArgs {
    #[arg(value_enum)]
    pub(crate) target: CliExportTarget,
    pub(crate) input: PathBuf,
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}
