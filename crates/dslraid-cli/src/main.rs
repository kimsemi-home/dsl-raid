use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod dispatch;
mod support;

pub(crate) use cli::*;
pub(crate) use support::*;

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    dispatch::run(cli.command)
}
