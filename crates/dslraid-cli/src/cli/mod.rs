mod command;
mod common;
mod formats;
mod graph;
mod ops;
mod outputs;
mod runtime;

use clap::Parser;

pub(crate) use command::Command;
pub(crate) use common::*;
pub(crate) use formats::*;
pub(crate) use graph::*;
pub(crate) use ops::*;
pub(crate) use outputs::*;
pub(crate) use runtime::*;

#[derive(Debug, Parser)]
#[command(
    name = "dslraid",
    version,
    about = "Executable architecture IR browser CLI"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}
