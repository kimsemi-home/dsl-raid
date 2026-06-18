mod diagnostics;
mod empty;
mod expand;
mod focus;
mod id;
mod initial;
mod input;
mod materialize;
mod mode;
mod output;
mod result;
mod run_cmd;
mod select;
mod tuple;
mod value;

#[cfg(test)]
mod tests;

use crate::OutputFormat;
use std::path::Path;

pub(crate) use result::result;
pub(crate) use run_cmd::run;

pub(crate) struct RunOptions<'a> {
    pub(crate) input: &'a Path,
    pub(crate) composition: Option<&'a str>,
    pub(crate) materialize: &'a str,
    pub(crate) limit: usize,
    pub(crate) focus: Option<&'a str>,
    pub(crate) depth: usize,
    pub(crate) format: OutputFormat,
    pub(crate) out: Option<&'a Path>,
}
