use crate::{write_bytes, write_or_stdout, OutputFormat};
use anyhow::{anyhow, bail, Context, Result};
use dslraid_core::{load_core_ir, sha256_json, state_subject, transition_subject};
use dslraid_core::{validate_json_schema, CoreIr};
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
    let trimmed = source.trim_start();
    let is_jsonl = input.extension().and_then(|ext| ext.to_str()) == Some("jsonl");
    let mut trace = if !is_jsonl && trimmed.starts_with('{') {
        serde_json::from_str::<Value>(&source)
            .with_context(|| format!("parse {}", input.display()))?
    } else {
        import_jsonl_trace(input, run_id, &source)?
    };

    if let Some(design_ir) = design_ir {
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
    }

    let temp_path =
        std::env::temp_dir().join(format!("dslraid-trace-import-{}.json", std::process::id()));
    write_bytes(&temp_path, serde_json::to_string_pretty(&trace)?.as_bytes())?;
    let issues = validate_json_schema(Path::new("schemas/dslraid-trace.schema.json"), &temp_path)?;
    fs::remove_file(&temp_path).ok();
    if !issues.is_empty() {
        for issue in &issues {
            println!("schema error at {}: {}", issue.instance_path, issue.message);
        }
        bail!("imported trace failed schema validation");
    }

    write_or_stdout(out, serde_json::to_string_pretty(&trace)?.as_bytes())
}

pub(crate) fn check(trace: &Path, design_ir: &Path, format: OutputFormat) -> Result<()> {
    let schema_issues =
        validate_json_schema(Path::new("schemas/dslraid-trace.schema.json"), trace)?;
    if !schema_issues.is_empty() {
        match format {
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&schema_issues)?),
            OutputFormat::Text => {
                for issue in &schema_issues {
                    println!("schema error at {}: {}", issue.instance_path, issue.message);
                }
            }
        }
        bail!("trace schema validation failed");
    }
    if matches!(format, OutputFormat::Text) {
        println!("schema ok: {}", trace.display());
    }

    let ir = load_core_ir(design_ir)?;
    let trace_value: Value = serde_json::from_slice(&fs::read(trace)?)?;
    let issues = trace_design_issues(&ir, &trace_value)?;
    let report = serde_json::json!({
        "status": if issues.is_empty() { "passed" } else { "failed" },
        "trace": trace.display().to_string(),
        "design_ir": design_ir.display().to_string(),
        "issues": issues
    });

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => print_trace_check_text(&report),
    }

    if report.get("status").and_then(Value::as_str) == Some("passed") {
        Ok(())
    } else {
        bail!("trace check failed")
    }
}

fn import_jsonl_trace(input: &Path, run_id: Option<&str>, source: &str) -> Result<Value> {
    let mut events = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut event: Value = serde_json::from_str(line)
            .with_context(|| format!("parse JSONL line {}", index + 1))?;
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
        events.push(event);
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

fn trace_design_issues(ir: &CoreIr, trace_value: &Value) -> Result<Vec<Value>> {
    let known_subjects = ir.semantic_subjects();
    let mut issues = Vec::new();

    for event in trace_value
        .get("events")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("trace.events must be an array"))?
    {
        let event_id = event
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>");
        let kind = event
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>");
        for field in ["subject", "from", "to"] {
            if let Some(subject) = event.get(field).and_then(Value::as_str) {
                if !known_subjects.contains(subject) {
                    issues.push(serde_json::json!({
                        "code": "RTE049",
                        "event": event_id,
                        "field": field,
                        "subject": subject,
                        "message": "runtime trace event does not map to a known design subject"
                    }));
                }
            }
        }
        if matches!(
            kind,
            "transition_started" | "transition_completed" | "transition_failed"
        ) {
            let Some(subject) = event.get("subject").and_then(Value::as_str) else {
                issues.push(serde_json::json!({
                    "code": "RTE049",
                    "event": event_id,
                    "message": "transition trace event is missing subject"
                }));
                continue;
            };
            if let Some((from, to)) = transition_endpoints(ir, subject) {
                if event
                    .get("from")
                    .and_then(Value::as_str)
                    .is_some_and(|value| value != from)
                {
                    issues.push(serde_json::json!({
                        "code": "RTE050",
                        "event": event_id,
                        "subject": subject,
                        "message": "trace from-state contradicts transition definition",
                        "expected": from,
                        "actual": event.get("from")
                    }));
                }
                if event
                    .get("to")
                    .and_then(Value::as_str)
                    .is_some_and(|value| value != to)
                {
                    issues.push(serde_json::json!({
                        "code": "RTE050",
                        "event": event_id,
                        "subject": subject,
                        "message": "trace to-state contradicts transition definition",
                        "expected": to,
                        "actual": event.get("to")
                    }));
                }
            }
        }
    }

    Ok(issues)
}

fn transition_endpoints(ir: &CoreIr, subject: &str) -> Option<(String, String)> {
    for fsm in &ir.fsms {
        for transition in &fsm.transitions {
            if transition_subject(&fsm.id, &transition.id) == subject {
                return Some((
                    state_subject(&fsm.id, &transition.from),
                    state_subject(&fsm.id, &transition.to),
                ));
            }
        }
    }
    None
}

fn print_trace_check_text(report: &Value) {
    if report.get("status").and_then(Value::as_str) == Some("passed") {
        println!("trace check passed");
    } else {
        println!("trace check failed");
        if let Some(issues) = report.get("issues").and_then(Value::as_array) {
            for issue in issues {
                println!(
                    "{} {}: {}",
                    issue
                        .get("code")
                        .and_then(Value::as_str)
                        .unwrap_or("RTE000"),
                    issue
                        .get("event")
                        .and_then(Value::as_str)
                        .unwrap_or("<unknown>"),
                    issue.get("message").and_then(Value::as_str).unwrap_or("")
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn transition_endpoints_resolves_fixture_transition() {
        let ir = load_core_ir(runscope_fixture()).unwrap();

        assert_eq!(
            transition_endpoints(&ir, "transition:runtime.idle_to_starting"),
            Some((
                "state:runtime.idle".to_string(),
                "state:runtime.starting".to_string()
            ))
        );
    }

    #[test]
    fn trace_design_issues_detects_contradicting_to_state() {
        let ir = load_core_ir(runscope_fixture()).unwrap();
        let trace = serde_json::json!({
            "events": [{
                "id": "evt-test",
                "kind": "transition_completed",
                "subject": "transition:runtime.idle_to_starting",
                "from": "state:runtime.idle",
                "to": "state:runtime.running"
            }]
        });

        let issues = trace_design_issues(&ir, &trace).unwrap();

        assert!(issues
            .iter()
            .any(|issue| issue.get("code").and_then(Value::as_str) == Some("RTE050")));
    }

    fn runscope_fixture() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/runscope/runscope.raid.json")
    }
}
