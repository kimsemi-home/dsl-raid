use anyhow::Result;
use dslraid_core::{CORE_SCHEMA_PATH, VALIDATION_SCHEMA_PATH};
use std::path::Path;

pub(super) fn check() -> Result<()> {
    for (schema, input) in fixture_schemas() {
        crate::schema_validate(Path::new(schema), Path::new(input))?;
    }
    Ok(())
}

fn fixture_schemas() -> [(&'static str, &'static str); 15] {
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
        (
            "schemas/dslraid-agent-run.schema.json",
            "examples/runscope/runscope.agent-run.json",
        ),
        (
            "schemas/dslraid-verification-evidence.schema.json",
            "docs/generated/verification-evidence.json",
        ),
        (
            "schemas/dslraid-verification-manifest.schema.json",
            "docs/generated/verification-privacy.json",
        ),
        (
            "schemas/dslraid-verification-manifest.schema.json",
            "docs/generated/verification-pdca.json",
        ),
        (
            "schemas/dslraid-verification-manifest.schema.json",
            "docs/generated/verification-loss-ledger.json",
        ),
        (
            "schemas/dslraid-verification-manifest.schema.json",
            "docs/generated/verification-semantic-hash.json",
        ),
        (
            "schemas/dslraid-verification-manifest.schema.json",
            "docs/generated/verification-codegen.json",
        ),
    ]
}
