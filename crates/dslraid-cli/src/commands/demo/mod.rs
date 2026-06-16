mod names;
mod package;

#[cfg(test)]
mod tests;

use anyhow::Result;

pub(crate) fn run(args: crate::DemoArgs) -> Result<()> {
    match args.command {
        crate::DemoCommand::Package { input, out, trace } => {
            package::run(&input, &out, trace.as_deref())
        }
    }
}
