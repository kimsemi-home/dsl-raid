use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::DefinedAt;

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
