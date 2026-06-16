#[derive(Debug, Clone)]
pub struct ValidateOptions {
    pub source_path: String,
    pub ir_hash: Option<String>,
    pub mode: String,
    pub deny: Vec<String>,
}

impl Default for ValidateOptions {
    fn default() -> Self {
        Self {
            source_path: "<memory>".to_string(),
            ir_hash: None,
            mode: "validate".to_string(),
            deny: Vec::new(),
        }
    }
}
