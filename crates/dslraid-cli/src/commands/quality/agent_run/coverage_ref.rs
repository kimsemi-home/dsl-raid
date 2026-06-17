use super::fields::{field_is, field_text, items};
use serde_json::Value;
use std::path::Path;

pub(super) fn push_issues(value: &Value, root: Option<&Path>, issues: &mut Vec<String>) {
    let Some(root) = root else {
        return;
    };
    for item in items(value, "evidence").filter(|item| field_is(item, "kind", "coverage")) {
        push_coverage_issue(root, item, issues);
    }
}

fn push_coverage_issue(root: &Path, evidence: &Value, issues: &mut Vec<String>) {
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
    }
}
