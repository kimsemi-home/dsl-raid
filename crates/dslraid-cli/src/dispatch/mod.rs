mod core;
mod graph;
mod outputs;
mod runtime;

use crate::Command;
use anyhow::Result;

pub(crate) fn run(command: Command) -> Result<()> {
    match command {
        Command::Init(args) => core::init(args),
        Command::Normalize(args) => core::normalize(args),
        Command::Migrate(args) => core::migrate(args),
        Command::Validate(args) => core::validate(args),
        Command::Schema(args) => core::schema(args),
        Command::Quality => crate::commands::quality::run(),
        Command::Golden(args) => core::golden(args),
        Command::Project(args) => outputs::project(args),
        Command::Render(args) => outputs::render(args),
        Command::Codegen(args) => outputs::codegen(args),
        Command::Doc(args) => outputs::doc(args),
        Command::Compose(args) => graph::compose(args),
        Command::Diff(args) => graph::diff(args),
        Command::Query(args) => graph::query(args),
        Command::Trace(args) => runtime::trace(args),
        Command::Coverage(args) => runtime::coverage(args),
        Command::Artifact(args) => runtime::artifact(args),
        Command::Compat(args) => core::compat(args),
        Command::Export(args) => outputs::export(args),
    }
}
