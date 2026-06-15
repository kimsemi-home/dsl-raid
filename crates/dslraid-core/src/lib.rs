use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

pub const CORE_SCHEMA_PATH: &str = "schemas/dslraid-core.schema.json";
pub const VALIDATION_SCHEMA_PATH: &str = "schemas/dslraid-validation.schema.json";
pub const VIEW_SCHEMA_PATH: &str = "schemas/dslraid-view.schema.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreIr {
    pub ir_version: String,
    pub project: Project,
    #[serde(default)]
    pub contexts: Vec<ContextObject>,
    #[serde(default)]
    pub requirements: Vec<Requirement>,
    #[serde(default)]
    pub capabilities: Vec<Capability>,
    #[serde(default)]
    pub policies: Vec<Policy>,
    #[serde(default)]
    pub commands: Vec<Command>,
    #[serde(default)]
    pub fsms: Vec<Fsm>,
    #[serde(default)]
    pub compositions: Vec<Composition>,
    #[serde(default)]
    pub projections: Vec<Projection>,
    #[serde(default)]
    pub derivations: Vec<Derivation>,
    #[serde(default)]
    pub artifacts: Vec<Artifact>,
    #[serde(default)]
    pub diagnostics: Vec<CoreDiagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub uid: Option<String>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextObject {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub owns: Vec<String>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub satisfied_by: Vec<String>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub provides: Vec<String>,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub applies_to: Vec<String>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub capability: Option<String>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fsm {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub context: Option<String>,
    #[serde(default)]
    pub states: Vec<State>,
    #[serde(default)]
    pub events: Vec<Event>,
    #[serde(default)]
    pub guards: Vec<Guard>,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default)]
    pub transitions: Vec<Transition>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub initial: bool,
    #[serde(default)]
    pub terminal: bool,
    #[serde(default)]
    pub terminal_semantics: Option<String>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guard {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub capability: Option<String>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub capability: Option<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub id: String,
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub on: Option<String>,
    #[serde(default)]
    pub guards: Vec<String>,
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Composition {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(default)]
    pub state_space: Option<Value>,
    #[serde(default)]
    pub conflict_policy: Option<Value>,
    #[serde(default)]
    pub projection: Option<Value>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projection {
    pub id: String,
    pub kind: String,
    pub source: String,
    #[serde(default)]
    pub show: Vec<String>,
    #[serde(default)]
    pub filters: Option<Value>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Derivation {
    pub id: String,
    pub source: String,
    pub rule: DerivationRule,
    #[serde(default)]
    pub targets: Vec<DerivationTarget>,
    #[serde(default)]
    pub defined_at: Option<DefinedAt>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivationRule {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub generator: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivationTarget {
    pub artifact: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub kind: String,
    pub path: String,
    #[serde(default)]
    pub generated_by: Option<String>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDiagnostic {
    pub id: String,
    pub code: String,
    pub severity: String,
    pub message: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    #[serde(default)]
    pub suggestion: Option<String>,
    #[serde(default)]
    pub evidence: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinedAt {
    pub uri: String,
    #[serde(default)]
    pub range: Option<SourceRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRange {
    #[serde(default)]
    pub start_line: Option<u32>,
    #[serde(default)]
    pub start_column: Option<u32>,
    #[serde(default)]
    pub end_line: Option<u32>,
    #[serde(default)]
    pub end_column: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaIssue {
    pub schema: String,
    pub instance: String,
    pub instance_path: String,
    pub message: String,
}

impl CoreIr {
    pub fn find_fsm(&self, id: &str) -> Option<&Fsm> {
        self.fsms.iter().find(|fsm| fsm.id == id)
    }

    pub fn find_projection(&self, id: Option<&str>) -> Option<&Projection> {
        match id {
            Some(id) => self
                .projections
                .iter()
                .find(|projection| projection.id == id),
            None => self.projections.first(),
        }
    }

    pub fn artifact_by_id(&self, id: &str) -> Option<&Artifact> {
        self.artifacts.iter().find(|artifact| artifact.id == id)
    }

    pub fn derivation_by_id(&self, id: &str) -> Option<&Derivation> {
        self.derivations
            .iter()
            .find(|derivation| derivation.id == id)
    }

    pub fn semantic_subjects(&self) -> BTreeSet<String> {
        let mut subjects = BTreeSet::new();
        subjects.insert(format!("project:{}", self.project.id));
        for context in &self.contexts {
            subjects.insert(context.id.clone());
        }
        for requirement in &self.requirements {
            subjects.insert(requirement.id.clone());
        }
        for capability in &self.capabilities {
            subjects.insert(capability.id.clone());
        }
        for policy in &self.policies {
            subjects.insert(policy.id.clone());
        }
        for command in &self.commands {
            subjects.insert(command.id.clone());
        }
        for fsm in &self.fsms {
            subjects.insert(fsm.id.clone());
            for state in &fsm.states {
                subjects.insert(state_subject(&fsm.id, &state.id));
            }
            for event in &fsm.events {
                subjects.insert(event_subject(&fsm.id, &event.id));
            }
            for guard in &fsm.guards {
                subjects.insert(guard_subject(&fsm.id, &guard.id));
            }
            for action in &fsm.actions {
                subjects.insert(action_subject(&fsm.id, &action.id));
            }
            for transition in &fsm.transitions {
                subjects.insert(transition_subject(&fsm.id, &transition.id));
            }
        }
        for composition in &self.compositions {
            subjects.insert(composition.id.clone());
        }
        for projection in &self.projections {
            subjects.insert(projection.id.clone());
        }
        for derivation in &self.derivations {
            subjects.insert(derivation.id.clone());
        }
        for artifact in &self.artifacts {
            subjects.insert(artifact.id.clone());
        }
        for diagnostic in &self.diagnostics {
            subjects.insert(diagnostic.id.clone());
        }
        subjects
    }
}

impl Fsm {
    pub fn local_name(&self) -> &str {
        fsm_local_name(&self.id)
    }

    pub fn state_subject(&self, state: &str) -> String {
        state_subject(&self.id, state)
    }

    pub fn transition_subject(&self, transition: &str) -> String {
        transition_subject(&self.id, transition)
    }
}

pub fn load_core_ir(path: impl AsRef<Path>) -> Result<CoreIr> {
    let path = path.as_ref();
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse core IR {}", path.display()))
}

pub fn load_json_value(path: impl AsRef<Path>) -> Result<Value> {
    let path = path.as_ref();
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse JSON {}", path.display()))
}

pub fn canonical_json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    serde_json::to_vec_pretty(value).context("failed to serialize canonical JSON")
}

pub fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("sha256:{}", hex::encode(digest))
}

pub fn sha256_json<T: Serialize>(value: &T) -> Result<String> {
    Ok(sha256_bytes(&canonical_json_bytes(value)?))
}

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

pub fn repo_schema_path(repo_root: impl AsRef<Path>, schema: &str) -> PathBuf {
    repo_root.as_ref().join(schema)
}

pub fn fsm_local_name(fsm_id: &str) -> &str {
    fsm_id.strip_prefix("fsm:").unwrap_or(fsm_id)
}

pub fn state_subject(fsm_id: &str, state_id: &str) -> String {
    format!("state:{}.{}", fsm_local_name(fsm_id), state_id)
}

pub fn event_subject(fsm_id: &str, event_id: &str) -> String {
    format!("event:{}.{}", fsm_local_name(fsm_id), event_id)
}

pub fn guard_subject(fsm_id: &str, guard_id: &str) -> String {
    format!("guard:{}.{}", fsm_local_name(fsm_id), guard_id)
}

pub fn action_subject(fsm_id: &str, action_id: &str) -> String {
    format!("action:{}.{}", fsm_local_name(fsm_id), action_id)
}

pub fn transition_subject(fsm_id: &str, transition_id: &str) -> String {
    format!("transition:{}.{}", fsm_local_name(fsm_id), transition_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_state_subjects() {
        assert_eq!(
            state_subject("fsm:runtime", "running"),
            "state:runtime.running"
        );
    }
}
