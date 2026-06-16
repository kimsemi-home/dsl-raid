mod command;
mod common;
mod demo;
mod docs;
mod formats;
mod generate;
mod graph;
mod ops;
mod outputs;
mod runtime;

use clap::Parser;

pub(crate) use command::Command;
pub(crate) use common::*;
pub(crate) use demo::*;
pub(crate) use docs::*;
pub(crate) use formats::*;
pub(crate) use generate::*;
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
