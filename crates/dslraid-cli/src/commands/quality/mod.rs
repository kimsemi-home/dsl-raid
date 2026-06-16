mod demo;
mod docs;
mod generated;
mod lisp;
mod lock;
mod projection;
mod roadmap;
mod runtime;
mod schema;
mod semantic;
mod source_shape;

use anyhow::Result;
use std::path::Path;

pub(crate) fn run() -> Result<()> {
    let input = Path::new("examples/runscope/runscope.raid.json");
    source_shape::check()?;
    schema::check_fixtures()?;
    let ir = semantic::check(input)?;
    projection::check(input, &ir)?;
    generated::check(input, &ir)?;
    lisp::check()?;
    demo::check(input)?;
    docs::check(input)?;
    roadmap::check()?;
    lock::check(input)?;
    runtime::check(input)?;
    crate::commands::artifact::verify(input, None, crate::OutputFormat::Text)?;
    println!("quality ok");
    Ok(())
}
