use serde::Serialize;
use serde_json::Value;
use std::path::Path;

#[derive(Serialize)]
pub(super) struct LockFile {
    pub(super) lock_version: &'static str,
    pub(super) core: CoreRecord,
    pub(super) resolver: ToolRecord,
    pub(super) resolved_refs: Vec<Value>,
    pub(super) derivations: Vec<DerivationRecord>,
    pub(super) artifacts: Vec<ArtifactRecord>,
    pub(super) golden: Vec<Value>,
    pub(super) compat: Vec<Value>,
}

#[derive(Serialize)]
pub(super) struct CoreRecord {
    path: String,
    schema_version: &'static str,
    ir_hash: String,
    hash_algorithm: &'static str,
}

#[derive(Serialize)]
pub(super) struct ToolRecord {
    pub(super) name: String,
    pub(super) version: String,
}

#[derive(Serialize)]
pub(super) struct DerivationRecord {
    pub(super) derivation: String,
    pub(super) rule: String,
    pub(super) generator: ToolRecord,
    pub(super) input_hash: String,
    pub(super) targets: Vec<String>,
}

#[derive(Serialize)]
pub(super) struct ArtifactRecord {
    pub(super) artifact: String,
    pub(super) path: String,
    pub(super) kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) generated_by: Option<String>,
    pub(super) input_hash: String,
    pub(super) content_hash: String,
    pub(super) schema_version: &'static str,
    pub(super) status: &'static str,
}

impl CoreRecord {
    pub(super) fn new(path: &Path, ir_hash: &str) -> Self {
        Self {
            path: path.display().to_string(),
            schema_version: "0.1.0",
            ir_hash: ir_hash.to_string(),
            hash_algorithm: "sha256",
        }
    }
}
