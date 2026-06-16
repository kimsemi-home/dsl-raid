use serde::{Deserialize, Serialize};

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
