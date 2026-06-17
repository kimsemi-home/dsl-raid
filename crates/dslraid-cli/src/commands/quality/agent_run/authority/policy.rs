use super::super::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if text(value, &["authority_gate", "policy_hash"]).is_none() {
        issues.push("approved authority gate requires policy hash".to_string());
    }
    if text(value, &["authority_gate", "approved_by"]).is_none() {
        issues.push("approved authority gate requires approver".to_string());
    }
    if value
        .pointer("/authority_gate/human_review_required")
        .and_then(Value::as_bool)
        .is_none()
    {
        issues.push("approved authority gate requires human review flag".to_string());
    }
}
