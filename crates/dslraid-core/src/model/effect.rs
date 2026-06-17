use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::DefinedAt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guard {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default = "default_guard_kind")]
    pub kind: String,
    #[serde(default)]
    pub capability: Option<String>,
    #[serde(default)]
    pub expression: Option<Value>,
    #[serde(default)]
    pub input: Option<String>,
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
    #[serde(default = "default_action_kind")]
    pub kind: String,
    #[serde(default)]
    pub capability: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub emits: Vec<String>,
    #[serde(default)]
    pub expression: Option<Value>,
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

fn default_guard_kind() -> String {
    "predicate".to_string()
}

fn default_action_kind() -> String {
    "effect".to_string()
}
