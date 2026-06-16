use anyhow::Result;
use std::path::Path;

pub(crate) fn validate(schema: &Path, input: &Path) -> Result<()> {
    crate::schema_validate(schema, input)
}
