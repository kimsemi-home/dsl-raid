use crate::{commands, ComposeArgs, DiffArgs, QueryArgs};
use anyhow::Result;

pub(super) fn compose(args: ComposeArgs) -> Result<()> {
    commands::compose::run(commands::compose::RunOptions {
        input: &args.input,
        composition: args.composition.as_deref(),
        materialize: &args.materialize,
        limit: args.limit,
        focus: args.focus.as_deref(),
        depth: args.depth,
        format: args.format,
        out: args.out.as_deref(),
    })
}

pub(super) fn diff(args: DiffArgs) -> Result<()> {
    commands::diff::run(&args.base, &args.head, args.format, args.out.as_deref())
}

pub(super) fn query(args: QueryArgs) -> Result<()> {
    commands::query::run(&args.input, &args.expression, args.format)
}
