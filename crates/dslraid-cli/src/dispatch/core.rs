use crate::{
    commands, GoldenArgs, GoldenCommand, InitArgs, MigrateArgs, NormalizeArgs, SchemaArgs,
    SchemaCommand, ValidateArgs,
};
use anyhow::Result;

pub(super) fn init(args: InitArgs) -> Result<()> {
    commands::workspace::init_project(&args.out)
}

pub(super) fn normalize(args: NormalizeArgs) -> Result<()> {
    commands::workspace::normalize(&args.input, args.out.as_deref())
}

pub(super) fn migrate(args: MigrateArgs) -> Result<()> {
    commands::workspace::migrate(&args.input, &args.from, &args.to, args.out.as_deref())
}

pub(super) fn validate(args: ValidateArgs) -> Result<()> {
    commands::validate::run(&args.input, &args.schema, args.format, args.deny)
}

pub(super) fn schema(args: SchemaArgs) -> Result<()> {
    match args.command {
        SchemaCommand::Validate { schema, input } => commands::schema::validate(&schema, &input),
    }
}

pub(super) fn golden(args: GoldenArgs) -> Result<()> {
    match args.command {
        GoldenCommand::Check { path } => commands::golden::check(&path),
        GoldenCommand::Update { path } => commands::golden::update(&path),
    }
}

pub(super) fn compat(args: crate::CompatArgs) -> Result<()> {
    match args.command {
        crate::CompatCommand::Check { input } => commands::workspace::compat_check(&input),
    }
}
