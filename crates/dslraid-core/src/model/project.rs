use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Artifact, Composition, CoreDiagnostic, Derivation, Fsm, Projection};
use super::{Capability, Command, ContextObject, Policy, Requirement};

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
