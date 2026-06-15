use crate::{validate_json_file, write_bytes, write_or_stdout, OutputFormat};
use anyhow::{anyhow, bail, Result};
use dslraid_core::{
    event_subject, load_core_ir, sha256_json, state_subject, transition_subject,
    validate_json_schema, CoreIr,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

pub(crate) fn build(trace: &Path, design_ir: &Path, out: Option<&Path>) -> Result<()> {
    validate_json_file(Path::new("schemas/dslraid-trace.schema.json"), trace)?;
    let ir = load_core_ir(design_ir)?;
    let trace_value: Value = serde_json::from_slice(&fs::read(trace)?)?;
    let coverage = coverage_overlay_value(&ir, design_ir, trace, &trace_value)?;
    let temp_path = std::env::temp_dir().join(format!(
        "dslraid-coverage-build-{}.json",
        std::process::id()
    ));
    write_bytes(
        &temp_path,
        serde_json::to_string_pretty(&coverage)?.as_bytes(),
    )?;
    validate_json_file(
        Path::new("schemas/dslraid-coverage.schema.json"),
        &temp_path,
    )?;
    fs::remove_file(&temp_path).ok();
    write_or_stdout(out, serde_json::to_string_pretty(&coverage)?.as_bytes())
}

pub(crate) fn check(coverage: &Path, design_ir: &Path, format: OutputFormat) -> Result<()> {
    let schema_issues =
        validate_json_schema(Path::new("schemas/dslraid-coverage.schema.json"), coverage)?;
    if !schema_issues.is_empty() {
        match format {
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&schema_issues)?),
            OutputFormat::Text => {
                for issue in &schema_issues {
                    println!("schema error at {}: {}", issue.instance_path, issue.message);
                }
            }
        }
        bail!("coverage schema validation failed");
    }

    let ir = load_core_ir(design_ir)?;
    let coverage_value: Value = serde_json::from_slice(&fs::read(coverage)?)?;
    let issues = coverage_design_issues(&ir, &coverage_value)?;
    let report = serde_json::json!({
        "status": if issues.is_empty() { "passed" } else { "failed" },
        "coverage": coverage.display().to_string(),
        "design_ir": design_ir.display().to_string(),
        "issues": issues
    });

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => print_coverage_check_text(&report),
    }

    if report.get("status").and_then(Value::as_str) == Some("passed") {
        Ok(())
    } else {
        bail!("coverage check failed")
    }
}

fn coverage_design_issues(ir: &CoreIr, coverage_value: &Value) -> Result<Vec<Value>> {
    let known_subjects = ir.semantic_subjects();
    let mut issues = Vec::new();
    let mut covered_subjects = BTreeSet::new();
    for subject in coverage_value
        .get("subjects")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("coverage.subjects must be an array"))?
    {
        let Some(subject_id) = subject.get("subject").and_then(Value::as_str) else {
            continue;
        };
        if !known_subjects.contains(subject_id) {
            issues.push(serde_json::json!({
                "code": "COV001",
                "subject": subject_id,
                "message": "Coverage subject does not resolve to the design IR."
            }));
        }
        covered_subjects.insert(subject_id.to_string());
    }
    for fsm in &ir.fsms {
        for state in &fsm.states {
            let subject = state_subject(&fsm.id, &state.id);
            if !covered_subjects.contains(&subject) {
                issues.push(serde_json::json!({
                    "code": "COV002",
                    "subject": subject,
                    "message": "Coverage overlay is missing a state subject."
                }));
            }
        }
        for transition in &fsm.transitions {
            let subject = transition_subject(&fsm.id, &transition.id);
            if !covered_subjects.contains(&subject) {
                issues.push(serde_json::json!({
                    "code": "COV002",
                    "subject": subject,
                    "message": "Coverage overlay is missing a transition subject."
                }));
            }
        }
    }
    Ok(issues)
}

#[derive(Debug, Clone)]
struct CoverageCounter {
    kind: String,
    count: usize,
    failures: usize,
    status_override: Option<String>,
    last_seen: Option<String>,
}

fn coverage_overlay_value(
    ir: &CoreIr,
    design_ir: &Path,
    trace: &Path,
    trace_value: &Value,
) -> Result<Value> {
    let mut counters = base_coverage_counters(ir);
    for event in trace_value
        .get("events")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("trace.events must be an array"))?
    {
        let kind = event
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let timestamp = event
            .get("timestamp")
            .and_then(Value::as_str)
            .map(str::to_string);
        let failed = event
            .get("status")
            .and_then(Value::as_str)
            .is_some_and(|status| {
                matches!(
                    status,
                    "failed" | "timeout" | "cancelled" | "policy_blocked" | "degraded"
                )
            })
            || kind == "transition_failed";
        match kind {
            "event_received"
            | "state_entered"
            | "state_exited"
            | "transition_started"
            | "transition_completed"
            | "transition_failed"
            | "action_started"
            | "action_completed"
            | "diagnostic_emitted" => {
                if let Some(subject) = event.get("subject").and_then(Value::as_str) {
                    mark_coverage(&mut counters, subject, failed, timestamp.clone(), None);
                }
                if matches!(
                    kind,
                    "transition_started" | "transition_completed" | "transition_failed"
                ) {
                    for field in ["from", "to"] {
                        if let Some(subject) = event.get(field).and_then(Value::as_str) {
                            mark_coverage(&mut counters, subject, false, timestamp.clone(), None);
                        }
                    }
                }
            }
            "artifact_deployed" => {
                if let Some(subject) = event.get("subject").and_then(Value::as_str) {
                    mark_coverage(
                        &mut counters,
                        subject,
                        failed,
                        timestamp.clone(),
                        Some("deployed"),
                    );
                }
            }
            _ => {}
        }
    }
    let mut subjects = counters
        .into_iter()
        .filter_map(|(subject, counter)| coverage_subject_value(subject, counter))
        .collect::<Vec<_>>();
    subjects.sort_by_key(|left| value_string(left, "subject"));
    Ok(serde_json::json!({
        "coverage_version": "0.1.0",
        "design_ir": {
            "path": design_ir.display().to_string(),
            "hash": sha256_json(ir)?
        },
        "traces": [{
            "path": trace.display().to_string(),
            "hash": sha256_json(trace_value)?
        }],
        "subjects": subjects,
        "metadata": {
            "generator": "dslraid-cli",
            "mode": "trace-derived"
        }
    }))
}

fn base_coverage_counters(ir: &CoreIr) -> BTreeMap<String, CoverageCounter> {
    let mut counters = BTreeMap::new();
    for fsm in &ir.fsms {
        for state in &fsm.states {
            counters.insert(
                state_subject(&fsm.id, &state.id),
                CoverageCounter::new("state"),
            );
        }
        for event in &fsm.events {
            counters.insert(
                event_subject(&fsm.id, &event.id),
                CoverageCounter::new("event"),
            );
        }
        for guard in &fsm.guards {
            counters.insert(
                format!("guard:{}.{}", fsm.local_name(), guard.id),
                CoverageCounter::new("guard"),
            );
        }
        for action in &fsm.actions {
            counters.insert(
                format!("action:{}.{}", fsm.local_name(), action.id),
                CoverageCounter::new("action"),
            );
        }
        for transition in &fsm.transitions {
            counters.insert(
                transition_subject(&fsm.id, &transition.id),
                CoverageCounter::new("transition"),
            );
        }
    }
    for artifact in &ir.artifacts {
        counters.insert(artifact.id.clone(), CoverageCounter::new("artifact"));
    }
    counters
}

impl CoverageCounter {
    fn new(kind: &str) -> Self {
        Self {
            kind: kind.to_string(),
            count: 0,
            failures: 0,
            status_override: None,
            last_seen: None,
        }
    }
}

fn mark_coverage(
    counters: &mut BTreeMap<String, CoverageCounter>,
    subject: &str,
    failed: bool,
    timestamp: Option<String>,
    status_override: Option<&str>,
) {
    if let Some(counter) = counters.get_mut(subject) {
        counter.count += 1;
        if failed {
            counter.failures += 1;
        }
        if let Some(status_override) = status_override {
            counter.status_override = Some(status_override.to_string());
        }
        if let Some(timestamp) = timestamp {
            counter.last_seen = Some(timestamp);
        }
    }
}

fn coverage_subject_value(subject: String, counter: CoverageCounter) -> Option<Value> {
    if !matches!(
        counter.kind.as_str(),
        "state" | "transition" | "event" | "guard" | "action" | "artifact"
    ) {
        return None;
    }
    let status = if let Some(status) = counter.status_override {
        status
    } else if counter.kind == "artifact" {
        if counter.count > 0 {
            "deployed".to_string()
        } else {
            "not_deployed".to_string()
        }
    } else if counter.failures > 0 {
        "failed".to_string()
    } else if counter.count > 0 {
        "covered".to_string()
    } else {
        "uncovered".to_string()
    };
    let failure_rate = if counter.count == 0 {
        0.0
    } else {
        counter.failures as f64 / counter.count as f64
    };
    let mut value = serde_json::json!({
        "subject": subject,
        "kind": counter.kind,
        "status": status,
        "count": counter.count,
        "failure_rate": failure_rate
    });
    if let Some(last_seen) = counter.last_seen {
        value
            .as_object_mut()
            .expect("coverage subject is an object")
            .insert("last_seen".to_string(), Value::String(last_seen));
    }
    Some(value)
}

fn print_coverage_check_text(report: &Value) {
    if report.get("status").and_then(Value::as_str) == Some("passed") {
        println!("coverage check passed");
    } else {
        println!("coverage check failed");
        if let Some(issues) = report.get("issues").and_then(Value::as_array) {
            for issue in issues {
                println!(
                    "{} {}: {}",
                    issue
                        .get("code")
                        .and_then(Value::as_str)
                        .unwrap_or("COV000"),
                    issue
                        .get("subject")
                        .and_then(Value::as_str)
                        .unwrap_or("<unknown>"),
                    issue.get("message").and_then(Value::as_str).unwrap_or("")
                );
            }
        }
    }
}

fn value_string(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn coverage_overlay_marks_fixture_transition_covered() {
        let ir = load_core_ir(runscope_fixture()).unwrap();
        let trace_path = runscope_trace();
        let trace_value: Value = serde_json::from_slice(&fs::read(&trace_path).unwrap()).unwrap();

        let coverage =
            coverage_overlay_value(&ir, &runscope_fixture(), &trace_path, &trace_value).unwrap();

        let subjects = coverage
            .get("subjects")
            .and_then(Value::as_array)
            .expect("coverage subjects are present");
        assert!(subjects.iter().any(|subject| {
            subject.get("subject").and_then(Value::as_str)
                == Some("transition:runtime.running_to_completed")
                && subject.get("status").and_then(Value::as_str) == Some("covered")
        }));
    }

    #[test]
    fn coverage_design_issues_reports_missing_subject() {
        let ir = load_core_ir(runscope_fixture()).unwrap();
        let coverage = serde_json::json!({
            "subjects": [{
                "subject": "transition:runtime.idle_to_starting"
            }]
        });

        let issues = coverage_design_issues(&ir, &coverage).unwrap();

        assert!(issues
            .iter()
            .any(|issue| issue.get("code").and_then(Value::as_str) == Some("COV002")));
    }

    fn runscope_fixture() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/runscope/runscope.raid.json")
    }

    fn runscope_trace() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/runscope/run-001.trace.json")
    }
}
