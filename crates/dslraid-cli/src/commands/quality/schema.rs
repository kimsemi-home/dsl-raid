use anyhow::{Context, Result};
use dslraid_core::{CORE_SCHEMA_PATH, VALIDATION_SCHEMA_PATH};
use serde_json::Value;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub(super) fn check_fixtures() -> Result<()> {
    check_json_syntax("schemas")?;
    check_json_syntax("examples")?;
    for (schema, input) in fixture_schemas() {
        crate::schema_validate(Path::new(schema), Path::new(input))?;
    }
    Ok(())
}

fn fixture_schemas() -> [(&'static str, &'static str); 8] {
    [
        (CORE_SCHEMA_PATH, "examples/runscope/runscope.raid.json"),
        (
            "schemas/dslraid-assertion.schema.json",
            "examples/runscope/runscope.assertions.json",
        ),
        (
            VALIDATION_SCHEMA_PATH,
            "examples/runscope/runscope.validation.json",
        ),
        (
            "schemas/dslraid-lock.schema.json",
            "examples/runscope/runscope.lock.json",
        ),
        (
            "schemas/dslraid-annotation.schema.json",
            "examples/runscope/runscope.annotations.json",
        ),
        (
            "schemas/dslraid-sourcemap.schema.json",
            "examples/runscope/runscope.sourcemap.json",
        ),
        (
            "schemas/dslraid-trace.schema.json",
            "examples/runscope/run-001.trace.json",
        ),
        (
            "schemas/dslraid-coverage.schema.json",
            "examples/runscope/run-001.coverage.json",
        ),
    ]
}

fn check_json_syntax(path: impl AsRef<Path>) -> Result<()> {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            let _: Value = serde_json::from_slice(
                &fs::read(path).with_context(|| format!("read {}", path.display()))?,
            )
            .with_context(|| format!("parse {}", path.display()))?;
        }
    }
    Ok(())
}
