use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub(crate) struct GenerateArgs {
    /// Canonical IR input file.
    pub(crate) input: PathBuf,
    /// Also regenerate the generated CLI reference Markdown.
    #[arg(long)]
    pub(crate) cli_doc: Option<PathBuf>,
    /// Also regenerate a validation proposition report JSON.
    #[arg(long)]
    pub(crate) validation_report: Option<PathBuf>,
    /// Do not refresh the inferred lock file after generation.
    #[arg(long)]
    pub(crate) skip_lock: bool,
}
