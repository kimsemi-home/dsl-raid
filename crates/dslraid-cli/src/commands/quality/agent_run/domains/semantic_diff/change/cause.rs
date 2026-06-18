use serde_json::Value;

const CAUSE_KINDS: &[&str] = &["observation", "trace", "debt", "artifact", "review"];

pub(super) fn push_issues(value: &Value, diff_id: &str, refs: &[&str], issues: &mut Vec<String>) {
    if refs.iter().any(|reference| {
        CAUSE_KINDS
            .iter()
            .any(|kind| super::has_kind(value, reference, kind))
    }) {
        return;
    }
    issues.push(format!(
        "changed semantic diff {diff_id} requires cause evidence"
    ));
}
