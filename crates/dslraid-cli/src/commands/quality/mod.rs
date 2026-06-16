mod demo;
mod docs;
mod generated;
mod lock;
mod projection;
mod runtime;
mod schema;
mod semantic;

use anyhow::Result;
use std::path::Path;

pub(crate) fn run() -> Result<()> {
    let input = Path::new("examples/runscope/runscope.raid.json");
    schema::check_fixtures()?;
    let ir = semantic::check(input)?;
    projection::check(input, &ir)?;
    generated::check(input, &ir)?;
    demo::check(input)?;
    docs::check(input)?;
    lock::check(input)?;
    runtime::check(input)?;
    crate::commands::artifact::verify(input, None, crate::OutputFormat::Text)?;
    println!("quality ok");
    Ok(())
}
