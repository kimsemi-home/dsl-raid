use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::DefinedAt;

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
