use serde::{Deserialize, Serialize};
use serde_json::Value;

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
pub struct SchemaIssue {
    pub schema: String,
    pub instance: String,
    pub instance_path: String,
    pub message: String,
}
