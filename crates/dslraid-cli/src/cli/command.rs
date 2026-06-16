use super::*;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Init(InitArgs),
    Normalize(NormalizeArgs),
    Migrate(MigrateArgs),
    Validate(ValidateArgs),
    Schema(SchemaArgs),
    Quality,
    Golden(GoldenArgs),
    Project(ProjectArgs),
    Render(RenderArgs),
    Codegen(CodegenArgs),
    Doc(DocArgs),
    Compose(ComposeArgs),
    Diff(DiffArgs),
    Query(QueryArgs),
    Trace(TraceArgs),
    Coverage(CoverageArgs),
    Artifact(ArtifactArgs),
    Compat(CompatArgs),
    Export(ExportArgs),
}
