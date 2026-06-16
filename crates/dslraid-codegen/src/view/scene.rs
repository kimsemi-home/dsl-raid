use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewNode {
    pub id: String,
    pub subject: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub label: String,
    #[serde(default)]
    pub badges: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewEdge {
    pub id: String,
    pub subject: String,
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub route: Vec<Point>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleToken {
    pub tone: String,
    pub emphasis: String,
}
