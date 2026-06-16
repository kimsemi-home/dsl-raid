use dslraid_core::{DefinedAt, SourceRange};
use serde_json::{json, Map, Value};

pub(super) fn dsl(defined_at: &Option<DefinedAt>) -> Option<Value> {
    let defined_at = defined_at.as_ref()?;
    let mut object = Map::new();
    object.insert("uri".to_string(), Value::String(defined_at.uri.clone()));
    if let Some(range) = source_range(&defined_at.range) {
        object.insert("range".to_string(), range);
    }
    Some(Value::Object(object))
}

pub(super) fn generated(artifact: &dslraid_core::Artifact, start: usize, end: usize) -> Value {
    json!({
        "artifact": artifact.id,
        "location": {
            "uri": artifact.path,
            "range": { "start_line": start, "end_line": end }
        }
    })
}

fn source_range(range: &Option<SourceRange>) -> Option<Value> {
    let range = range.as_ref()?;
    Some(json!({
        "start_line": range.start_line?,
        "end_line": range.end_line?,
    }))
}
