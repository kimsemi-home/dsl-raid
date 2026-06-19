use super::fixture::diff;
use serde_json::json;

#[test]
fn semantic_diff_rejects_unknown_evidence() {
    let mut value = super::manifest();
    value["semantic_diffs"] = json!([diff("sha256:core", json!(["evidence:missing"]))]);

    assert_eq!(
        super::issues(&value),
        vec!["semantic diff semantic-diff:quality references unknown evidence evidence:missing"]
    );
}

#[test]
fn changed_semantic_diff_requires_validation_evidence() {
    let mut value = super::manifest();
    value["semantic_diffs"] = json!([diff("sha256:core", json!(["evidence:trace"]))]);

    assert_eq!(
        super::issues(&value),
        vec!["changed semantic diff semantic-diff:quality requires validation evidence"]
    );
}
