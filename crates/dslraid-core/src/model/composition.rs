use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::DefinedAt;

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
