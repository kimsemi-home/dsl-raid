use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiffChange {
    pub(crate) action: &'static str,
    pub(crate) kind: String,
    pub(crate) subject: String,
    pub(crate) label: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) before: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) after: Option<Value>,
}
