use serde_json::json;

#[test]
fn changed_semantic_diff_requires_cause_evidence() {
    let mut value = super::manifest();
    value["semantic_diffs"] = json!([super::fixture::diff(
        "sha256:core",
        json!(["evidence:quality"])
    )]);

    assert_eq!(
        super::issues(&value),
        vec!["changed semantic diff semantic-diff:quality requires cause evidence"]
    );
}
