mod evidence;
mod manifest;
mod ontology;
mod refs;
mod selection;
mod shadow;
mod verifier;

use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let Some(item) = value.get("orchestration") else {
        issues.push("approved run requires orchestration receipt".to_string());
        return;
    };
    manifest::push_issues(value, item, issues);
    ontology::push_issues(value, item, issues);
    evidence::push_issues(value, item, issues);
    refs::push_issues(value, item, issues);
    selection::push_issues(value, item, issues);
    shadow::push_issues(value, item, issues);
    verifier::push_issues(value, item, issues);
}
