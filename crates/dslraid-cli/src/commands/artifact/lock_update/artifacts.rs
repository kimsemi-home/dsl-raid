use dslraid_core::{Artifact, CoreIr};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

use super::model::ArtifactRecord;

pub(super) fn records(
    ir: &CoreIr,
    input: &Path,
    current_hash: &str,
    existing: &BTreeMap<String, Value>,
) -> Vec<ArtifactRecord> {
    ir.artifacts
        .iter()
        .filter(|artifact| super::super::kind::requires_lock_record(artifact))
        .map(|artifact| record(artifact, input, current_hash, existing))
        .collect()
}

fn record(
    artifact: &Artifact,
    input: &Path,
    current_hash: &str,
    existing: &BTreeMap<String, Value>,
) -> ArtifactRecord {
    let actual_hash = super::super::path::artifact_content_hash(input, &artifact.path);
    ArtifactRecord {
        artifact: artifact.id.clone(),
        path: artifact.path.clone(),
        kind: artifact.kind.clone(),
        generated_by: artifact.generated_by.clone(),
        input_hash: current_hash.to_string(),
        content_hash: actual_hash.unwrap_or_else(|| fallback_hash(existing, &artifact.id)),
        schema_version: "0.1.0",
        status: if file_exists(input, &artifact.path) {
            "fresh"
        } else {
            "missing"
        },
    }
}

fn fallback_hash(existing: &BTreeMap<String, Value>, artifact: &str) -> String {
    existing
        .get(artifact)
        .and_then(|record| record.get("content_hash"))
        .and_then(Value::as_str)
        .unwrap_or("sha256:0000000000000000000000000000000000000000000000000000000000000000")
        .to_string()
}

fn file_exists(input: &Path, artifact_path: &str) -> bool {
    super::super::path::resolve_artifact_path(input, artifact_path).exists()
}
