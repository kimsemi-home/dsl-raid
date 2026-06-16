use anyhow::{anyhow, bail, Context, Result};
use serde_json::Value;
use std::path::Path;

pub(super) fn import_jsonl_trace(
    input: &Path,
    run_id: Option<&str>,
    source: &str,
) -> Result<Value> {
    let mut events = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        events.push(parse_jsonl_event(index, line)?);
    }

    Ok(serde_json::json!({
        "trace_version": "0.1.0",
        "run": {
            "id": run_id
                .map(str::to_string)
                .unwrap_or_else(|| input.file_stem().and_then(|stem| stem.to_str()).unwrap_or("imported-run").to_string()),
            "environment": "imported"
        },
        "events": events
    }))
}

fn parse_jsonl_event(index: usize, line: &str) -> Result<Value> {
    let mut event: Value =
        serde_json::from_str(line).with_context(|| format!("parse JSONL line {}", index + 1))?;
    let object = event
        .as_object_mut()
        .ok_or_else(|| anyhow!("JSONL line {} must be an object", index + 1))?;
    object
        .entry("id".to_string())
        .or_insert_with(|| Value::String(format!("evt-{:04}", index + 1)));
    if !object.contains_key("timestamp") {
        bail!("JSONL line {} is missing timestamp", index + 1);
    }
    if !object.contains_key("kind") {
        bail!("JSONL line {} is missing kind", index + 1);
    }
    Ok(event)
}
