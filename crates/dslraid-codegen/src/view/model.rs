use serde::{Deserialize, Serialize};

use super::{InspectorPanel, ViewEdge, ViewNode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewModel {
    pub view_version: String,
    pub source: ViewSource,
    pub layout: Layout,
    pub nodes: Vec<ViewNode>,
    pub edges: Vec<ViewEdge>,
    #[serde(default)]
    pub inspector_panels: Vec<InspectorPanel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewSource {
    pub core_ir: String,
    pub projection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub engine: String,
    pub version: String,
}
