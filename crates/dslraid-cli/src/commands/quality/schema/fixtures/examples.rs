use super::Fixture;
use dslraid_core::{CORE_SCHEMA_PATH, VALIDATION_SCHEMA_PATH};

pub(super) fn schemas() -> [Fixture; 9] {
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
    ]
}
