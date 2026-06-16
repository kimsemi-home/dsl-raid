use anyhow::{bail, Result};
use dslraid_core::{load_core_ir, validate_json_schema};
use std::path::Path;

pub(crate) fn run(
    input: &Path,
    schema: &Path,
    format: crate::OutputFormat,
    deny: Vec<String>,
) -> Result<()> {
    let schema_issues = validate_json_schema(schema, input)?;
    if !schema_issues.is_empty() {
        match format {
            crate::OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&schema_issues)?)
            }
            crate::OutputFormat::Text => print_schema_issues(&schema_issues),
        }
        bail!(
            "schema validation failed with {} issues",
            schema_issues.len()
        );
    }

    let ir = load_core_ir(input)?;
    let report = crate::validation_report(&ir, input, "validate", deny.clone())?;
    match format {
        crate::OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        crate::OutputFormat::Text => crate::print_report_text(&report),
    }
    if !report.is_success(&deny) {
        bail!("validation failed");
    }
    Ok(())
}

fn print_schema_issues(schema_issues: &[dslraid_core::SchemaIssue]) {
    for issue in schema_issues {
        println!("schema error at {}: {}", issue.instance_path, issue.message);
    }
}
