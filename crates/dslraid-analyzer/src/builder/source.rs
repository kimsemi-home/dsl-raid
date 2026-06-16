use crate::{ValidationRun, ValidationSource, TOOL_VERSION};

pub(crate) fn source(source_path: String, ir_hash: Option<String>) -> ValidationSource {
    ValidationSource {
        core_ir: source_path,
        ir_hash,
        lock: None,
        assertions: None,
        projection: None,
    }
}

pub(crate) fn run(mode: String, deny: Vec<String>) -> ValidationRun {
    ValidationRun {
        tool: "dslraid-cli".to_string(),
        version: Some(TOOL_VERSION.to_string()),
        mode,
        deny,
    }
}
