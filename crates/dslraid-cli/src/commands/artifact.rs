use crate::{validate_json_file, OutputFormat};
use anyhow::{bail, Result};
use dslraid_core::{load_core_ir, sha256_json};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn verify(input: &Path, lock: Option<&Path>, format: OutputFormat) -> Result<()> {
    let report = artifact_verify_report(input, lock)?;
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => print_artifact_verify_text(&report),
    }
    if report.get("status").and_then(Value::as_str) == Some("passed") {
        Ok(())
    } else {
        bail!("artifact verification failed")
    }
}

fn artifact_verify_report(input: &Path, lock: Option<&Path>) -> Result<Value> {
    let ir = load_core_ir(input)?;
    let lock_path = lock
        .map(Path::to_path_buf)
        .unwrap_or_else(|| inferred_lock_path(input));
    validate_json_file(
        &repo_relative_path("schemas/dslraid-lock.schema.json"),
        &lock_path,
    )?;
    let lock_value: Value = serde_json::from_slice(&fs::read(&lock_path)?)?;
    let current_ir_hash = sha256_json(&ir)?;
    let mut issues = Vec::new();
    let mut artifact_results = Vec::new();

    let lock_core_hash = lock_value
        .get("core")
        .and_then(|core| core.get("ir_hash"))
        .and_then(Value::as_str);
    if lock_core_hash != Some(current_ir_hash.as_str()) {
        issues.push(artifact_issue(
            "ART038",
            "error",
            &format!("project:{}", ir.project.id),
            "lock core hash differs from current IR hash",
            lock_core_hash,
            Some(current_ir_hash.as_str()),
        ));
    }

    let lock_artifacts = lock_artifact_map(&lock_value);
    let derivation_inputs = lock_derivation_input_map(&lock_value);
    for derivation in &ir.derivations {
        match derivation_inputs.get(&derivation.id) {
            Some(input_hash) if input_hash == &current_ir_hash => {}
            Some(input_hash) => issues.push(artifact_issue(
                "ART039",
                "error",
                &derivation.id,
                "derivation input hash differs from current IR hash",
                Some(input_hash.as_str()),
                Some(current_ir_hash.as_str()),
            )),
            None => issues.push(artifact_issue(
                "ART040",
                "error",
                &derivation.id,
                "derivation is missing from lock file",
                None,
                Some("locked derivation record"),
            )),
        }
    }

    for artifact in &ir.artifacts {
        let mut artifact_status = "fresh";
        if artifact.kind == "generated" || artifact.kind == "test" || artifact.kind == "doc" {
            match artifact.generated_by.as_deref() {
                Some(derivation) if ir.derivation_by_id(derivation).is_some() => {}
                Some(derivation) => {
                    artifact_status = "stale";
                    issues.push(artifact_issue(
                        "ART034",
                        "error",
                        &artifact.id,
                        "artifact references an unknown derivation",
                        Some(derivation),
                        Some("existing derivation"),
                    ));
                }
                None => {
                    artifact_status = "stale";
                    issues.push(artifact_issue(
                        "ART034",
                        "error",
                        &artifact.id,
                        "generated artifact has no generated_by derivation",
                        None,
                        Some("generated_by"),
                    ));
                }
            }
        }

        match lock_artifacts.get(&artifact.id) {
            Some(record) => {
                if record.get("path").and_then(Value::as_str) != Some(artifact.path.as_str()) {
                    artifact_status = "stale";
                    issues.push(artifact_issue(
                        "ART041",
                        "error",
                        &artifact.id,
                        "artifact path differs from lock file",
                        record.get("path").and_then(Value::as_str),
                        Some(artifact.path.as_str()),
                    ));
                }
                if record.get("kind").and_then(Value::as_str) != Some(artifact.kind.as_str()) {
                    artifact_status = "stale";
                    issues.push(artifact_issue(
                        "ART041",
                        "error",
                        &artifact.id,
                        "artifact kind differs from lock file",
                        record.get("kind").and_then(Value::as_str),
                        Some(artifact.kind.as_str()),
                    ));
                }
                if record.get("generated_by").and_then(Value::as_str)
                    != artifact.generated_by.as_deref()
                {
                    artifact_status = "stale";
                    issues.push(artifact_issue(
                        "ART041",
                        "error",
                        &artifact.id,
                        "artifact generated_by differs from lock file",
                        record.get("generated_by").and_then(Value::as_str),
                        artifact.generated_by.as_deref(),
                    ));
                }
                let input_hash = record.get("input_hash").and_then(Value::as_str);
                if input_hash != Some(current_ir_hash.as_str()) {
                    artifact_status = "stale";
                    issues.push(artifact_issue(
                        "ART039",
                        "error",
                        &artifact.id,
                        "artifact input hash differs from current IR hash",
                        input_hash,
                        Some(current_ir_hash.as_str()),
                    ));
                }
                if record.get("status").and_then(Value::as_str) == Some("stale") {
                    artifact_status = "stale";
                    issues.push(artifact_issue(
                        "ART039",
                        "error",
                        &artifact.id,
                        "lock file marks artifact as stale",
                        Some("stale"),
                        Some("fresh"),
                    ));
                }
                if record.get("status").and_then(Value::as_str) == Some("missing") {
                    artifact_status = "missing";
                    issues.push(artifact_issue(
                        "ART040",
                        "error",
                        &artifact.id,
                        "lock file marks artifact as missing",
                        Some("missing"),
                        Some("fresh"),
                    ));
                }
            }
            None if artifact.kind == "generated"
                || artifact.kind == "test"
                || artifact.kind == "doc" =>
            {
                artifact_status = "missing";
                issues.push(artifact_issue(
                    "ART040",
                    "error",
                    &artifact.id,
                    "artifact is missing from lock file",
                    None,
                    Some("locked artifact record"),
                ));
            }
            None => {
                artifact_status = "external";
            }
        }
        artifact_results.push(serde_json::json!({
            "artifact": artifact.id,
            "path": artifact.path,
            "kind": artifact.kind,
            "generated_by": artifact.generated_by,
            "status": artifact_status
        }));
    }

    artifact_results.sort_by_key(|artifact| value_string(artifact, "artifact"));
    issues.sort_by_key(|issue| {
        format!(
            "{}:{}",
            value_string(issue, "code"),
            value_string(issue, "subject")
        )
    });
    Ok(serde_json::json!({
        "artifact_verify_version": "0.1.0",
        "status": if issues.is_empty() { "passed" } else { "failed" },
        "input": input.display().to_string(),
        "lock": lock_path.display().to_string(),
        "current_ir_hash": current_ir_hash,
        "artifacts": artifact_results,
        "issues": issues
    }))
}

fn inferred_lock_path(input: &Path) -> PathBuf {
    let Some(file_name) = input.file_name().and_then(|name| name.to_str()) else {
        return input.with_extension("lock.json");
    };
    let lock_name = if let Some(prefix) = file_name.strip_suffix(".raid.json") {
        format!("{prefix}.lock.json")
    } else if let Some(prefix) = file_name.strip_suffix(".dslraid.json") {
        format!("{prefix}.dslraid.lock.json")
    } else if let Some(prefix) = file_name.strip_suffix(".json") {
        format!("{prefix}.lock.json")
    } else {
        format!("{file_name}.lock.json")
    };
    input
        .parent()
        .map(|parent| parent.join(&lock_name))
        .unwrap_or_else(|| PathBuf::from(lock_name))
}

fn repo_relative_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.exists() {
        return direct;
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

fn lock_artifact_map(lock_value: &Value) -> BTreeMap<String, Value> {
    lock_value
        .get("artifacts")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|artifact| {
            artifact
                .get("artifact")
                .and_then(Value::as_str)
                .map(|id| (id.to_string(), artifact.clone()))
        })
        .collect()
}

fn lock_derivation_input_map(lock_value: &Value) -> BTreeMap<String, String> {
    lock_value
        .get("derivations")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|derivation| {
            let id = derivation.get("derivation").and_then(Value::as_str)?;
            let input_hash = derivation.get("input_hash").and_then(Value::as_str)?;
            Some((id.to_string(), input_hash.to_string()))
        })
        .collect()
}

fn artifact_issue(
    code: &'static str,
    severity: &'static str,
    subject: &str,
    message: &str,
    actual: Option<&str>,
    expected: Option<&str>,
) -> Value {
    serde_json::json!({
        "code": code,
        "severity": severity,
        "subject": subject,
        "message": message,
        "actual": actual,
        "expected": expected
    })
}

fn print_artifact_verify_text(report: &Value) {
    println!(
        "artifact verification {}",
        report
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
    );
    println!(
        "input: {}",
        report
            .get("input")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>")
    );
    println!(
        "lock: {}",
        report
            .get("lock")
            .and_then(Value::as_str)
            .unwrap_or("<unknown>")
    );
    if let Some(artifacts) = report.get("artifacts").and_then(Value::as_array) {
        let fresh = artifacts
            .iter()
            .filter(|artifact| artifact.get("status").and_then(Value::as_str) == Some("fresh"))
            .count();
        let stale = artifacts
            .iter()
            .filter(|artifact| artifact.get("status").and_then(Value::as_str) == Some("stale"))
            .count();
        let missing = artifacts
            .iter()
            .filter(|artifact| artifact.get("status").and_then(Value::as_str) == Some("missing"))
            .count();
        let external = artifacts
            .iter()
            .filter(|artifact| artifact.get("status").and_then(Value::as_str) == Some("external"))
            .count();
        println!("artifacts: fresh={fresh} stale={stale} missing={missing} external={external}");
    }
    if let Some(issues) = report.get("issues").and_then(Value::as_array) {
        for issue in issues {
            println!(
                "{} {} {}: {}",
                issue
                    .get("severity")
                    .and_then(Value::as_str)
                    .unwrap_or("error"),
                issue
                    .get("code")
                    .and_then(Value::as_str)
                    .unwrap_or("ART000"),
                issue
                    .get("subject")
                    .and_then(Value::as_str)
                    .unwrap_or("<unknown>"),
                issue.get("message").and_then(Value::as_str).unwrap_or("")
            );
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

    #[test]
    fn artifact_verify_passes_fixture_lock() {
        let report = artifact_verify_report(&runscope_fixture(), Some(&runscope_lock())).unwrap();

        assert_eq!(report.get("status").and_then(Value::as_str), Some("passed"));
        assert_eq!(
            report.get("issues").and_then(Value::as_array).map(Vec::len),
            Some(0)
        );
    }

    #[test]
    fn artifact_verify_detects_stale_input_hash() {
        let mut lock: Value = serde_json::from_slice(&fs::read(runscope_lock()).unwrap()).unwrap();
        lock["core"]["ir_hash"] = Value::String(
            "sha256:0000000000000000000000000000000000000000000000000000000000000001".to_string(),
        );
        lock["artifacts"][0]["input_hash"] = Value::String(
            "sha256:0000000000000000000000000000000000000000000000000000000000000001".to_string(),
        );
        let temp = temp_lock_path();
        fs::write(&temp, serde_json::to_vec_pretty(&lock).unwrap()).unwrap();

        let report = artifact_verify_report(&runscope_fixture(), Some(&temp)).unwrap();
        fs::remove_file(&temp).ok();

        assert_eq!(report.get("status").and_then(Value::as_str), Some("failed"));
        let issues = report
            .get("issues")
            .and_then(Value::as_array)
            .expect("report issues are present");
        assert!(issues
            .iter()
            .any(|issue| issue.get("code").and_then(Value::as_str) == Some("ART038")));
        assert!(issues
            .iter()
            .any(|issue| issue.get("code").and_then(Value::as_str) == Some("ART039")));
    }

    fn runscope_fixture() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/runscope/runscope.raid.json")
    }

    fn runscope_lock() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples/runscope/runscope.lock.json")
    }

    fn temp_lock_path() -> PathBuf {
        std::env::temp_dir().join(format!(
            "dslraid-stale-lock-test-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }
}
