use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectorPanel {
    pub subject: String,
    pub title: String,
    pub sections: Vec<InspectorSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectorSection {
    pub title: String,
    pub rows: Vec<InspectorRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectorRow {
    pub label: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}
