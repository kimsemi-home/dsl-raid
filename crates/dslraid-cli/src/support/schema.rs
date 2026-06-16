use anyhow::{bail, Result};
use dslraid_core::validate_json_schema;
use std::path::Path;

pub(crate) fn schema_validate(schema: &Path, input: &Path) -> Result<()> {
    let issues = validate_json_schema(schema, input)?;
    if issues.is_empty() {
        println!("schema ok: {}", input.display());
        Ok(())
    } else {
        for issue in &issues {
            println!("schema error at {}: {}", issue.instance_path, issue.message);
        }
        bail!("schema validation failed with {} issues", issues.len())
    }
}

pub(crate) fn validate_json_file(schema: &Path, input: &Path) -> Result<()> {
    let issues = validate_json_schema(schema, input)?;
    if issues.is_empty() {
        Ok(())
    } else {
        bail!(
            "{} failed schema validation with {} issues",
            input.display(),
            issues.len()
        )
    }
}
