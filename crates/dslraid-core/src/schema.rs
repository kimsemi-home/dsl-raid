use anyhow::Result;
use std::path::Path;

use crate::{load_json_value, SchemaIssue};

pub fn validate_json_schema(
    schema_path: impl AsRef<Path>,
    instance_path: impl AsRef<Path>,
) -> Result<Vec<SchemaIssue>> {
    let schema_path = schema_path.as_ref();
    let instance_path = instance_path.as_ref();
    let schema = load_json_value(schema_path)?;
    let instance = load_json_value(instance_path)?;
    let compiled = jsonschema::JSONSchema::compile(&schema).map_err(|error| {
        anyhow::anyhow!(
            "failed to compile schema {}: {}",
            schema_path.display(),
            error
        )
    })?;
    let result = match compiled.validate(&instance) {
        Ok(()) => Ok(Vec::new()),
        Err(errors) => Ok(errors
            .map(|error| SchemaIssue {
                schema: schema_path.display().to_string(),
                instance: instance_path.display().to_string(),
                instance_path: error.instance_path.to_string(),
                message: error.to_string(),
            })
            .collect()),
    };
    result
}
