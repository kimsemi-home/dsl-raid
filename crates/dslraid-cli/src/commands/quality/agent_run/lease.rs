use super::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if text(value, &["lease", "status"]) != Some("finished") {
        issues.push("approved run requires finished lease".to_string());
    }
}
