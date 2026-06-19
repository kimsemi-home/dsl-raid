use anyhow::Result;
use std::path::Path;

const TRACE_JSONL: &str = "examples/runscope/run-002.trace.jsonl";
const SCHEMA: &str = "schemas/dslraid-trace.schema.json";

pub(super) fn check(input: &Path) -> Result<()> {
    let imported_trace = super::temp_file::path("dslraid-imported-trace", "json");
    crate::commands::trace::import(
        Path::new(TRACE_JSONL),
        Some(input),
        Some("run-002"),
        Some(&imported_trace),
    )?;
    crate::schema_validate(Path::new(SCHEMA), &imported_trace)?;
    super::temp_file::remove(&imported_trace);
    Ok(())
}
