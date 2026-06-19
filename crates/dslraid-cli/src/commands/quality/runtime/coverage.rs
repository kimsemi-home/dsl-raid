use anyhow::Result;
use std::path::Path;

const TRACE: &str = "examples/runscope/run-001.trace.json";
const SCHEMA: &str = "schemas/dslraid-coverage.schema.json";

pub(super) fn check(input: &Path) -> Result<()> {
    let coverage_path = super::temp_file::path("dslraid-coverage", "json");
    crate::commands::coverage::build(Path::new(TRACE), input, Some(&coverage_path))?;
    crate::schema_validate(Path::new(SCHEMA), &coverage_path)?;
    crate::commands::coverage::check(&coverage_path, input, crate::OutputFormat::Text)?;
    super::temp_file::remove(&coverage_path);
    Ok(())
}
