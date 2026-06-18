mod link;

use super::super::fields::{field_is, field_text, items};
use serde_json::Value;
use std::{fs, path::Path};

pub(super) fn push_issues(value: &Value, root: Option<&Path>, issues: &mut Vec<String>) {
    let Some(root) = root else {
        return;
    };
    let traces = link::trace_uris(value);
    for item in items(value, "evidence").filter(|item| field_is(item, "kind", "coverage")) {
        push_coverage_issue(value, root, item, &traces, issues);
    }
}

fn push_coverage_issue(
    manifest: &Value,
    root: &Path,
    evidence: &Value,
    traces: &link::TraceUris,
    issues: &mut Vec<String>,
) {
    let Some(uri) = field_text(evidence, "uri") else {
        return;
    };
    let path = root.join(uri);
    if !path.exists() {
        issues.push(format!("coverage evidence {uri} does not exist"));
        return;
    }
    if crate::validate_json_file(Path::new("schemas/dslraid-coverage.schema.json"), &path).is_err()
    {
        issues.push(format!("coverage evidence {uri} failed coverage schema"));
        return;
    }
    let Some(coverage) = read_json(&path) else {
        issues.push(format!("coverage evidence {uri} is unreadable"));
        return;
    };
    link::push_issues(manifest, &coverage, uri, traces, issues);
}

fn read_json(path: &Path) -> Option<Value> {
    fs::read(path)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
}
