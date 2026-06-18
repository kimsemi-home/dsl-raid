use super::super::fields::{field_is, field_text, items};
use serde_json::Value;
use std::path::Path;

pub(super) fn push_issues(value: &Value, root: Option<&Path>, issues: &mut Vec<String>) {
    let Some(root) = root else {
        return;
    };
    for item in items(value, "evidence").filter(|item| field_is(item, "kind", "trace")) {
        push_trace_issue(root, item, issues);
    }
}

fn push_trace_issue(root: &Path, evidence: &Value, issues: &mut Vec<String>) {
    let Some(uri) = field_text(evidence, "uri") else {
        return;
    };
    let path = root.join(uri);
    if !path.exists() {
        issues.push(format!("trace evidence {uri} does not exist"));
        return;
    }
    if crate::validate_json_file(Path::new("schemas/dslraid-trace.schema.json"), &path).is_err() {
        issues.push(format!("trace evidence {uri} failed trace schema"));
    }
}
