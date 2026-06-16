use super::{jsonl, output};
use crate::{write_bytes, write_or_stdout};
use anyhow::{anyhow, bail, Context, Result};
use dslraid_core::{load_core_ir, sha256_json, validate_json_schema};
use serde_json::Value;
use std::fs;
use std::path::Path;

pub(crate) fn import(
    input: &Path,
    design_ir: Option<&Path>,
    run_id: Option<&str>,
    out: Option<&Path>,
) -> Result<()> {
    let source = fs::read_to_string(input).with_context(|| format!("read {}", input.display()))?;
    let mut trace = parse_trace_input(input, run_id, &source)?;
    if let Some(design_ir) = design_ir {
        attach_design_ir(&mut trace, design_ir)?;
    }
    validate_imported_trace(&trace)?;
    write_or_stdout(out, serde_json::to_string_pretty(&trace)?.as_bytes())
}

fn parse_trace_input(input: &Path, run_id: Option<&str>, source: &str) -> Result<Value> {
    let trimmed = source.trim_start();
    let is_jsonl = input.extension().and_then(|ext| ext.to_str()) == Some("jsonl");
    if !is_jsonl && trimmed.starts_with('{') {
        serde_json::from_str::<Value>(source).with_context(|| format!("parse {}", input.display()))
    } else {
        jsonl::import_jsonl_trace(input, run_id, source)
    }
}

fn attach_design_ir(trace: &mut Value, design_ir: &Path) -> Result<()> {
    let hash = sha256_json(&load_core_ir(design_ir)?)?;
    trace
        .as_object_mut()
        .ok_or_else(|| anyhow!("trace root must be an object"))?
        .insert(
            "design_ir".to_string(),
            serde_json::json!({
                "path": design_ir.display().to_string(),
                "hash": hash
            }),
        );
    Ok(())
}

fn validate_imported_trace(trace: &Value) -> Result<()> {
    let temp_path =
        std::env::temp_dir().join(format!("dslraid-trace-import-{}.json", std::process::id()));
    write_bytes(&temp_path, serde_json::to_string_pretty(trace)?.as_bytes())?;
    let issues = validate_json_schema(Path::new("schemas/dslraid-trace.schema.json"), &temp_path)?;
    fs::remove_file(&temp_path).ok();
    if issues.is_empty() {
        return Ok(());
    }
    output::print_schema_issues(&issues);
    bail!("imported trace failed schema validation");
}
