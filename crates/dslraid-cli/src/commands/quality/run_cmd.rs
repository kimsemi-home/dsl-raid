use anyhow::Result;
use std::path::Path;

pub(crate) fn run() -> Result<()> {
    let input = Path::new("examples/runscope/runscope.raid.json");
    super::source_shape::check()?;
    super::schema::check_fixtures()?;
    check_agent_run()?;
    let ir = super::semantic::check(input)?;
    super::projection::check(input, &ir)?;
    super::generated::check(input, &ir)?;
    super::lisp::check()?;
    super::demo::check(input)?;
    super::docs::check(input)?;
    super::lock::check(input)?;
    super::runtime::check(input)?;
    crate::commands::artifact::verify(input, None, crate::OutputFormat::Text)?;
    println!("quality ok");
    Ok(())
}

fn check_agent_run() -> Result<()> {
    super::agent_run::check(
        Path::new("examples/runscope/runscope.agent-run.json"),
        Path::new("examples/runscope/runscope.lock.json"),
    )
}
